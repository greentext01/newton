use std::{
    sync::{Arc, RwLock},
    thread,
};

use clap::Parser;
use cli::arg_parser::Arguments;
use env_logger::Env;
use networking::server::{Config, Server};
use physics::{
    physics_engine::PhysicsEngine,
    physics_runner::{run_physics, PhysicsConfig},
    state_loader::load_state,
};

mod cli;
mod networking;
mod physics;

fn main() {
    let env = Env::default()
        .filter_or("NEWTON_LOG_LEVEL", "info")
        .write_style_or("NEWTON_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let arguments = Arguments::parse();

    let mut physics = PhysicsEngine::new(arguments.softening);

    let state = load_state(&arguments.system);

    let server_config = Config {
        network_interface: "0.0.0.0",
        network_port: 5000,
    };

    let inputs_rwlock = Arc::new(RwLock::new(state.inputs));
    let objects_rwlock = Arc::new(RwLock::new((state.ships, state.planets)));

    let inputs_rwlock_clone = inputs_rwlock.clone();
    let objects_rwlock_clone = objects_rwlock.clone();

    let physics_config = PhysicsConfig {
        target_updates_per_s: arguments.target_fps,
        min_spu: arguments.min_spu,
        max_spu: arguments.max_spu,
    };

    thread::spawn(move || {
        run_physics(
            &mut physics,
            objects_rwlock_clone,
            inputs_rwlock_clone,
            physics_config,
        );
    });

    let server = Server::new(server_config, objects_rwlock, inputs_rwlock, &arguments);
    if server.is_none() {
        return;
    }

    let server = server.unwrap();

    server.run();
}
