mod cli;
mod networking;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread,
};

use common::data::state::State;
use macroquad::prelude::*;
use networking::client::{Client, Config};

#[macroquad::main("Flight")]
async fn main() {
    env_logger::init();

    let state_lock: Arc<RwLock<Option<State>>> = Arc::new(RwLock::new(None));

    let quit_flag = Arc::new(AtomicBool::new(false));

    let client_config = Config {
        network_interface: "",
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
        log::info!("Network thread starting...");

        client.run();
    });

    loop {
        clear_background(BLACK);

        if quit_flag.load(Ordering::SeqCst) {
            log::trace!("Quitting from graphics thread...");
            break;
        }

        next_frame().await;
    }
}
