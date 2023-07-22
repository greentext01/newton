use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// A generic object in space.
/// Contains data for physics calculations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    /// Mass 
    pub mass: f64,
    
    /// Position in meters
    pub position: Array1<f64>,
    
    /// Velocity in meters per second
    pub velocity: Array1<f64>,
    
    /// Acceleration in meters per second squared
    pub acceleration: Array1<f64>,

    /// Heading in radians
    pub heading: f64,
    
    /// Spin in radians per second
    pub spin: f64,

    /// A unique identifier for the object.
    pub id: i32,

    /// The name of the texture for the object to use
    pub texture: String,
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
