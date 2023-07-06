use serde::{Serialize, Deserialize};

use super::inputs::Inputs;
use super::object::Planet;
use super::object::Ship;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub ships: Vec<Ship>,
    pub planets: Vec<Planet>,
    pub inputs: Inputs,
}
