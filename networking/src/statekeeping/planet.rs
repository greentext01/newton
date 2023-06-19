use ndarray::Array1;

use crate::proto::state::Planet as NetPlanet;

#[derive(Debug, Clone)]
pub struct Planet {
    pub mass: f64,
    pub position: Array1<f64>,
    pub heading: f64,
    pub velocity: Array1<f64>,
    pub radius: f64,
    pub spin: f64,
}

impl TryInto<Planet> for NetPlanet {
    type Error = ();

    fn try_into(self) -> Result<Planet, Self::Error> {
        let out = Planet {
            heading: self.heading,
            mass: self.mass,
            position: self.position.into(),
            spin: self.spin,
            velocity: self.velocity.into(),
            radius: self.radius,
        };

        Ok(out)
    }
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
