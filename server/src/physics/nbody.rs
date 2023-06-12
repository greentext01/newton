use ndarray::{s, Array2};

use networking::statekeeping::state::State;

use super::physics_engine::{AccelerationCalculator, ObjectStepper, PhysicsEngine};

pub struct NBodyPhysics {
    pub softening: f64,
    pub ship_accelerations: Array2<f64>,
    pub planet_accelerations: Array2<f64>,
}

impl NBodyPhysics {
    pub fn new(softening: Option<f64>, state: &State) -> NBodyPhysics {
        NBodyPhysics {
            softening: softening.unwrap_or(0.1),
            ship_accelerations: Array2::zeros((state.ships.len(), 2)),
            planet_accelerations: Array2::zeros((state.planets.len(), 2)),
        }
    }
}

impl AccelerationCalculator for NBodyPhysics {
    fn get_planet_accelerations(&self, state: &State) -> Array2<f64> {
        let mut planet_accelerations = Array2::zeros((state.planets.len(), 2));

        // TODO: Vectorize this if this is slow.
        // Note: Extracting this into its own function would require me to add a lot
        // of code to generalize both the Planet and the Ship t

        for (planet_i_index, planet_i) in state.planets.iter().enumerate() {
            for planet_j in &state.planets {
                let distance = &planet_j.position - &planet_i.position;
                let dx = distance[0];
                let dy = distance[1];

                let mut acceleration = planet_accelerations.slice_mut(s![planet_i_index, ..]);
                let acceleration_from_j = (planet_j.mass * (&distance))
                    * (dx.powi(2) + dy.powi(2) + self.softening.powi(2)).powf(-1.5);

                acceleration += &acceleration_from_j;
            }
        }

        return planet_accelerations;
    }

    fn get_ship_accelerations(&self, state: &State) -> Array2<f64> {
        let mut ship_accelerations = Array2::zeros((state.ships.len(), 2));

        // TODO: Vectorize this if this is slow.

        for (ship_index, ship) in state.ships.iter().enumerate() {
            for planet in &state.planets {
                let distance = &planet.position - &ship.position;
                let dx = distance[0];
                let dy = distance[1];

                // Needed to get the acceleration of the ship in the x and y axis, in a mutable ndarray.
                // This value gets operations peformed on it.
                let mut ship_acceleration = ship_accelerations.slice_mut(s![ship_index, ..]);
                let acceleration_from_j = (planet.mass * (&distance))
                    * (dx.powi(2) + dy.powi(2) + self.softening.powi(2)).powf(-1.5);

                ship_acceleration += &acceleration_from_j;
            }
        }

        return ship_accelerations;
    }
}

impl ObjectStepper for NBodyPhysics {
    /// Gets ships' acceleration, and integrates to get each ships' position
    fn step_ships(&mut self, state: &mut State, dt: f64) {
        for (i, ship) in state.ships.iter_mut().enumerate() {
            let acc = &self.ship_accelerations.slice(s![i, ..]);
            ship.velocity += &(acc * dt / 2.);
            ship.position += &(&ship.velocity * dt);
        }

        self.ship_accelerations = self.get_ship_accelerations(state);

        for (i, ship) in state.ships.iter_mut().enumerate() {
            let acc = &self.ship_accelerations.slice(s![i, ..]);
            ship.velocity += &(acc * dt / 2.);
        }
    }

    /// Gets planets' acceleration, and integrates to get each planets' position
    fn step_planets(&mut self, state: &mut State, dt: f64) {
        for (i, planet) in state.planets.iter_mut().enumerate() {
            let acc = &self.planet_accelerations.slice(s![i, ..]);
            planet.velocity += &(acc * dt / 2.);
            planet.position += &(&planet.velocity * dt);
        }

        self.planet_accelerations = self.get_planet_accelerations(state);

        for (i, planet) in state.planets.iter_mut().enumerate() {
            let acc = &self.planet_accelerations.slice(s![i, ..]);
            planet.velocity += &(acc * dt / 2.);
        }
    }
}

impl PhysicsEngine for NBodyPhysics {
    /// Integrates and gets the new positions.
    /// Uses leapfrog integration (see https://en.wikipedia.org/wiki/Leapfrog_integration#Algorithm)
    /// Kick-Drift-Kick form
    fn step(&mut self, state: &mut State, dt: f64, steps_per_frame: u32) {
        for _ in 0..steps_per_frame {
            self.step_planets(state, dt / steps_per_frame as f64);
            self.step_ships(state, dt / steps_per_frame as f64);
        }
    }
}
