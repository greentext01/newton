use ndarray::{s, Array2};

use common::data::{
    inputs::Inputs,
    object::{Objects, ShipType},
};

const G: f64 = 6.674e-11;

pub struct PhysicsEngine {
    pub softening: f64,
}

impl PhysicsEngine {
    pub fn new(softening: f64) -> PhysicsEngine {
        PhysicsEngine { softening }
    }

    fn get_planet_accelerations(&self, objects: &mut Objects) -> Array2<f64> {
        let (_, planets) = objects;
        let mut planet_accelerations = Array2::zeros((planets.len(), 2));

        // TODO: Vectorize this if this is slow.
        // Note: Extracting this into its own function would require me to add a lot
        // of code to generalize both the Planet and the Ship t

        for (planet_i_index, planet_i) in planets.iter().enumerate() {
            for (planet_j_index, planet_j) in planets.iter().enumerate() {
                if planet_i_index == planet_j_index {
                    continue;
                }

                let distance = &planet_j.object.position - &planet_i.object.position;
                let dx = distance[0];
                let dy = distance[1];

                // Needed to get the acceleration of the ship in the x and y axis, in a mutable ndarray.
                // This value gets operations peformed on it.
                let mut acceleration = planet_accelerations.slice_mut(s![planet_i_index, ..]);
                let inv_r3 = (dx.powi(2) + dy.powi(2) + self.softening.powi(2)).powf(-1.5);
                acceleration[0] += G * planet_j.object.mass * &(dx) * inv_r3;
                acceleration[1] += G * planet_j.object.mass * &(dy) * inv_r3;
            }
        }

        planet_accelerations
    }

    fn get_ship_accelerations(&self, objects: &Objects, inputs: &Inputs) -> Array2<f64> {
        let (ships, planets) = objects;
        let mut ship_accelerations = Array2::zeros((ships.len(), 2));

        // TODO: Vectorize this if this is slow.

        for (ship_index, ship) in ships.iter().enumerate() {
            for planet in planets {
                let distance = &planet.object.position - &ship.object.position;
                let dx = distance[0];
                let dy = distance[1];

                // Needed to get the acceleration of the ship in the x and y axis, in a mutable ndarray.
                // This value gets operations peformed on it.
                let mut ship_acceleration = ship_accelerations.slice_mut(s![ship_index, ..]);
                let inv_r3 = (dx.powi(2) + dy.powi(2) + self.softening.powi(2)).powf(-1.5);
                let acceleration_from_j = planet.object.mass * &(distance) * inv_r3;

                ship_acceleration += &acceleration_from_j;
            }
        }

        for (ship_index, ship) in ships.iter().enumerate() {
            match &ship.ship_type {
                ShipType::HAB(data) => {
                    let mut acceleration = ship_accelerations.slice_mut(s![ship_index, ..]);
                    let heading_vector =
                        ndarray::arr1(&[ship.object.heading.cos(), ship.object.heading.sin()]);
                    let acc_from_engine = heading_vector * data.thrust as f64 * inputs.throttle as f64;
                    acceleration += &acc_from_engine;
                }
            }
        }

        ship_accelerations
    }

    /// Gets ships' acceleration, and integrates to get each ships' position
    fn step_ships(&mut self, objects: &mut Objects, inputs: &Inputs, dt: f64) {
        for ship in objects.0.iter_mut() {
            ship.object.velocity += &(&ship.object.acceleration * dt / 2.);
            ship.object.position += &(&ship.object.velocity * dt);
        }

        let accelerations = self.get_ship_accelerations(objects, inputs);

        for (i, ship) in objects.0.iter_mut().enumerate() {
            ship.object.acceleration = accelerations.slice(s![i, ..]).to_owned();
        }

        for ship in objects.0.iter_mut() {
            ship.object.velocity += &(&ship.object.acceleration * dt / 2.);
        }
    }

    /// Gets planets' acceleration, and integrates to get each planets' position
    fn step_planets(&mut self, objects: &mut Objects, dt: f64) {
        // for (i, planet) in state.planets.iter_mut().enumerate() {
        //     let acc = &self.planet_accelerations.slice(s![i, ..]);
        //     planet.velocity += &(acc * dt / 2.);
        //     planet.position += &(&planet.velocity * dt);
        // }

        // for (i, planet) in state.planets.iter_mut().enumerate() {
        //     let acc = &self.planet_accelerations.slice(s![i, ..]);
        //     planet.velocity += &(acc * dt / 2.);
        // }
        let accelerations = self.get_planet_accelerations(objects);
        let (_, planets) = objects;

        for (i, planet) in planets.iter_mut().enumerate() {
            let acc = &accelerations.slice(s![i, ..]);
            planet.object.velocity += &(acc * dt);
            planet.object.position += &(&planet.object.velocity * dt);
        }
    }

    /// Integrates and gets the new positions.
    /// Uses leapfrog integration (see https://en.wikipedia.org/wiki/Leapfrog_integration#Algorithm)
    /// Kick-Drift-Kick form
    pub fn step(&mut self, objects: &mut Objects, inputs: &Inputs, dt: f64, steps_per_frame: u32) {
        for _ in 0..steps_per_frame {
            self.step_planets(objects, dt / steps_per_frame as f64);
            self.step_ships(objects, inputs, dt / steps_per_frame as f64);
        }
    }
}
