use ndarray::Array1;

use crate::proto::state::{Ship as NetShip};

#[derive(Debug, Clone)]
pub struct Ship {
    pub mass: f64,
    pub position: Array1<f64>,
    pub heading: f64,
    pub velocity: Array1<f64>,
    pub spin: f64,
    pub thrust: f64,
}

impl TryInto<Ship> for NetShip {
    type Error = ();

    fn try_into(self) -> Result<Ship, Self::Error> {
        let out = Ship {
            heading: self.heading,
            mass: self.mass,
            position: self.position.into(),
            spin: self.spin,
            velocity: self.velocity.into(),
            thrust: self.thrust,
        };

        Ok(out)
    }
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