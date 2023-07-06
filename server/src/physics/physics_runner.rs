use std::sync::{RwLock, Arc};

use common::data::{object::{Ship, Planet}, inputs::Inputs};

use super::physics_engine::PhysicsEngine;



pub struct PhysicsConfig {
    pub target_updates_per_s: u32,
    pub min_spu: u32,
    pub max_spu: Option<u32>,
}

pub fn run_physics(
    physics: &mut PhysicsEngine,
    objects_rwlock: Arc<RwLock<(Vec<Ship>, Vec<Planet>)>>,
    inputs_rwlock_clone: Arc<RwLock<Inputs>>,
    config: PhysicsConfig,
) {
    let mut steps_per_update: u32 = 5;
    let mut prev_time = std::time::Instant::now();
    let mut first_frame = true;

    loop {
        let now = std::time::Instant::now();
        let dt = now.duration_since(prev_time).as_secs_f64();
        prev_time = now;

        // dt can sometimes be 0, which causes a divide by zero error
        let fps = (if dt > 0.0 { 1.0 / dt } else { 0.0 }) as u32;
        log::trace!("FPS: {}", fps);

        if !first_frame {
            // Tune the number of steps to try to hit the target updates per second
            let spu_unclamped = (steps_per_update + fps - config.target_updates_per_s) as u32;

            if let Some(max) = config.max_spu {
                steps_per_update = spu_unclamped.clamp(config.min_spu, max);
            } else {
                steps_per_update = spu_unclamped.max(config.min_spu);
            }
        }
        log::trace!("SPU: {}", steps_per_update);

        first_frame = false;

        let objects_lock = objects_rwlock.read().unwrap();
        let mut objects = objects_lock.clone();
        drop(objects_lock);

        let inputs_lock = inputs_rwlock_clone.read().unwrap();
        let inputs = inputs_lock.clone();
        drop(inputs_lock);

        physics.step(&mut objects, &inputs, dt, steps_per_update);

        let mut object_w_lock = objects_rwlock.write().unwrap();
        *object_w_lock = objects;
    }
}