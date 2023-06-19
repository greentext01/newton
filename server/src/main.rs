use std::sync::Arc;

use crate::physics::physics_engine::PhysicsEngine;
use ndarray::array;
use networking::{
    proto::net_state_server::NetStateServer,
    statekeeping::{inputs::Inputs, planet::Planet, state::State},
    MyStateServer,
};
use physics::nbody::NBodyPhysics;
use tokio::sync::{mpsc, watch, Mutex};
use tonic::transport::Server;

mod physics;

const TARGET_UPDATES_PER_S: i32 = 60;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State {
        ships: vec![],
        planets: vec![
            Planet {
                heading: 0.0,
                mass: 100000.0,
                position: array![100.0, 100.0],
                radius: 10.0,
                spin: 0.0,
                velocity: array![20.0, 0.0],
            },
            Planet {
                heading: 0.0,
                mass: 100000.0,
                position: array![100.0, 200.0],
                radius: 10.0,
                spin: 0.0,
                velocity: array![-20.0, 0.0],
            },
            Planet {
                heading: 0.0,
                mass: 100000.0,
                position: array![300.0, 100.0],
                radius: 10.0,
                spin: 0.0,
                velocity: array![-20.0, 0.0],
            },
        ],
        inputs: Inputs::default(),
    };

    let mut physics = NBodyPhysics::new(None, &state);

    let mut steps_per_update: u32 = 2;
    let mut prev_time = std::time::Instant::now();

    let addr = "[::1]:50051".parse()?;
    let (state_tx, state_rx) = watch::channel(state.clone());
    let (command_queue_tx, command_queue_rx) = mpsc::channel(100);

    let server = MyStateServer {
        state: Arc::new(Mutex::new(state_rx)),
        command_queue_tx,
    };

    tokio::spawn(async move {
        Server::builder()
            .add_service(NetStateServer::new(server))
            .serve(addr)
            .await
            .unwrap_or_else(|error| {
                panic!("Server threw an error: {:?}", error)
            });
    });

    loop {
        let now = std::time::Instant::now();
        let dt = now.duration_since(prev_time).as_secs_f32();
        prev_time = now;

        let updates_per_s = (if dt > 0.0 { 1.0 / dt } else { 0.0 }) as i32;

        // Tune the number of steps to try to hit the target updates per second
        steps_per_update = (steps_per_update as i32 + (updates_per_s - TARGET_UPDATES_PER_S))
            .clamp(10, 1000) as u32;
        println!("{}", steps_per_update);

        physics.step(&mut state, dt as f64, steps_per_update);
        match state_tx.send(state.clone()) {
            Err(status) => println!("Error: State could not be sent ({})", status),
            _ => {}
        }
    }
}
