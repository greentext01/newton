use ndarray::Array2;

use networking::statekeeping::state::State;

pub trait PhysicsEngine {
    fn step(&mut self, state: &mut State, dt: f64, steps_per_frame: u32);
}

pub trait AccelerationCalculator {
    /// Gets accelerations of each ship.
    ///
    /// The output from this gets integrated, to get
    /// each position. This function does not take into
    /// account the insignificant forces applied by ships
    /// to planets.
    /// Gets accelerations of each planet.
    ///
    /// The output from this gets integrated, to get
    /// each position.
    fn get_planet_accelerations(&self, state: &State) -> Array2<f64>;
    
    /// Gets accelerations of each ship.
    ///
    /// The output from this gets integrated, to get
    /// each position. This function does not take into
    /// account the insignificant forces applied by ships
    /// to planets.
    /// Gets accelerations of each planet.
    ///
    /// The output from this gets integrated, to get
    /// each position.
    fn get_ship_accelerations(&self, state: &State) -> Array2<f64>;
}

pub trait ObjectStepper {
    fn step_ships(&mut self, state: &mut State, dt: f64);
    fn step_planets(&mut self, state: &mut State, dt: f64);
}
