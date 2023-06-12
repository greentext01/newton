use ndarray::Array1;

use crate::proto::state::{Inputs as NetInputs, Planet as NetPlanet, Ship as NetShip};
use crate::proto::State as NetState;

#[derive(Debug, Clone)]
pub struct State {
    pub ships: Vec<Ship>,
    pub planets: Vec<Planet>,
    pub inputs: Inputs,
}

impl Into<NetState> for State {
    fn into(self) -> NetState {
        NetState {
            inputs: Some(self.inputs.into()),
            planets: self.planets.into_iter().map(|p| p.into()).collect(),
            ships: self.ships.into_iter().map(|p| p.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NavMode {
    MAN,
}

#[derive(Debug, Clone)]
pub struct Inputs {
    pub navmode: NavMode,
    pub throttle: f32,
}

impl Into<NetInputs> for Inputs {
    fn into(self) -> NetInputs {
        NetInputs {
            navmode: self.navmode as i32,
            throttle: self.throttle,
        }
    }
}

impl Inputs {
    pub fn default() -> Inputs {
        Inputs {
            navmode: NavMode::MAN,
            throttle: 0.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub mass: f64,
    pub position: Array1<f64>,
    pub heading: f64,
    pub velocity: Array1<f64>,
    pub spin: f64,
    pub thrust: f64,
}

impl Into<NetShip> for Ship {
    fn into(self) -> NetShip {
        NetShip {
            heading: self.heading,
            mass: self.mass,
            position: self.position.to_vec(),
            spin: self.spin,
            velocity: self.velocity.to_vec(),
            thrust: self.thrust,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub mass: f64,
    pub position: Array1<f64>,
    pub heading: f64,
    pub velocity: Array1<f64>,
    pub radius: f64,
    pub spin: f64,
}

impl Into<NetPlanet> for Planet {
    fn into(self) -> NetPlanet {
        NetPlanet {
            heading: self.heading,
            mass: self.mass,
            position: self.position.to_vec(),
            spin: self.spin,
            velocity: self.velocity.to_vec(),
            radius: self.radius,
        }
    }
}
