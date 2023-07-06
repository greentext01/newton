use serde::{Serialize, Deserialize};

/// Dictates the desired rotation of the ship.
/// The name "NavMode" is carried over from Orbit.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NavMode {
    MAN,
}

/// Inputs given to the server by the client.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inputs {
    pub navmode: NavMode,
    pub throttle: f32,
}
