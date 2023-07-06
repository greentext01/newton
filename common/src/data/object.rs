use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// A generic object in space.
/// Contains data for physics calculations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    pub mass: f64,
    pub position: Array1<f64>,
    pub velocity: Array1<f64>,
    pub acceleration: Array1<f64>,
    pub heading: f64,
    pub spin: f64,
}

pub type Objects = (Vec<Ship>, Vec<Planet>);

// ----------------- PLANETS -----------------

/// A planet.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planet {
    pub object: Object,
    pub radius: f64,
}

// ----------------- SHIPS -----------------

/// Data specific to the HAB ship type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HabData {
    pub thrust: f32,
}

/// Enum containing the ship type, and data attached to it.
/// This is used to determine artificial ship acccelerations, and to add
/// custom code based on ship type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShipType {
    HAB(HabData),
}

/// Any ship
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ship {
    pub object: Object,
    pub ship_type: ShipType,
}

