mod cli;
mod networking;
mod graphics;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread,
};

use common::data::state::State;
use env_logger::Env;
use graphics::icon::*;
use macroquad::{miniquad::conf::Icon, prelude::*};
use networking::client::{Client, Config};

fn window_conf() -> Conf {
    Conf {
        window_title: "Flight".to_owned(),
        window_width: 800,
        window_height: 600,
        icon: Some(Icon {
            small: ICON_SMALL,
            medium: ICON_MEDIUM,
            big: ICON_BIG,
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let env = Env::default()
        .filter_or("FLIGHT_LOG_LEVEL", "info")
        .write_style_or("FLIGHT_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let state_lock: Arc<RwLock<Option<State>>> = Arc::new(RwLock::new(None));

    let quit_flag = Arc::new(AtomicBool::new(false));

    let client_config = Config {
        network_interface: "127.0.0.1",
        network_port: 5000,
    };

    let client = Client::new(
        client_config,
        Arc::clone(&state_lock),
        Arc::clone(&quit_flag),
    );

    if client.is_none() {
        return;
    }

    let client = client.unwrap();

    thread::spawn(move || {
        log::trace!("Network thread starting...");

        client.run();
    });

    loop {
        if quit_flag.load(Ordering::SeqCst) {
            log::trace!("Quitting from graphics thread...");
            break;
        }

        let state_guard = state_lock.read().unwrap();
        let state = state_guard.clone();
        drop(state_guard);

        if state.is_none() {
            next_frame().await;
            continue;
        }

        let state = state.unwrap();

        clear_background(BLACK);

        for planet in state.planets.iter() {
            draw_circle(
                planet.object.position[0] as f32,
                planet.object.position[1] as f32,
                planet.radius as f32,
                WHITE,
            );
        }

        next_frame().await;
    }
}
