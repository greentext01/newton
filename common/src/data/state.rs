use serde::{Serialize, Deserialize};

use super::inputs::Inputs;
use super::object::Object;
use super::object::Planet;
use super::object::Ship;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub ships: Vec<Ship>,
    pub planets: Vec<Planet>,
    pub inputs: Inputs,
}

impl State {
    /// Returns an array with the objects attribute of both ships and planets.
    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        self.ships.iter().map(|s| &s.object).chain(self.planets.iter().map(|p| &p.object))
    }
}
