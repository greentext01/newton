mod cli;
mod data;
mod graphics;
mod networking;

use std::{
    sync::{mpsc::channel, Arc, RwLock},
    thread,
};

use common::data::state::State;
use data::client_state::ClientState;
use env_logger::Env;
use macroquad::{miniquad::conf::Icon, prelude::*};
use networking::client::{Client, Config, NetThreadEvent};
use graphics::{icon::*, textures::Textures, renderer::Renderer};

fn config() -> Conf {
    Conf {
        window_title: "Flight".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: true,
        icon: Some(Icon {
            big: ICON_BIG,
            medium: ICON_MEDIUM,
            small: ICON_SMALL,
        }),
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let env = Env::default()
        .filter_or("FLIGHT_LOG_LEVEL", "info")
        .write_style_or("FLIGHT_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let state_lock: Arc<RwLock<Option<State>>> = Arc::new(RwLock::new(None));

    let client_config = Config {
        network_interface: "127.0.0.1",
        network_port: 5000,
    };

    let (events_tx, events_rx) = channel();

    let textures = Textures::new().await;

    let client = Client::new(client_config, Arc::clone(&state_lock), events_tx);

    if client.is_none() {
        return;
    }

    let client = client.unwrap();

    thread::spawn(move || {
        log::trace!("Network thread starting...");

        client.run();
    });

    let mut connected = false;

    let client_state = ClientState { center: 1 };

    let mut renderer = Renderer::new(&client_state, &textures);

    'outer: loop {
        'inner: loop {
            match events_rx.try_recv() {
                Err(..) => break 'inner,
                Ok(event) => match event {
                    NetThreadEvent::Quit => {
                        log::trace!("Quitting from graphics thread...");
                        break 'outer;
                    }
                    NetThreadEvent::Connected => {
                        log::info!("Connected to server");
                        connected = true;
                    }
                    NetThreadEvent::Disconnected => {
                        log::info!("Disconnected from server");
                        connected = false;
                    }
                },
            }
        }
        
        if !connected {
            clear_background(BLACK);
            
            // Draw splash screen
            renderer.draw_splash(textures.splash);

            next_frame().await;
            continue;
        }

        let state_guard = state_lock.read().unwrap();
        let state = state_guard.clone();
        drop(state_guard);

        if state.is_none() {
            next_frame().await;
            continue;
        }

        let state = state.unwrap();

        renderer.render(&state);

        next_frame().await;
    }
}
