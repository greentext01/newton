use crate::proto;

use super::inputs::Inputs;
use super::planet::Planet;
use super::ship::Ship;

#[derive(Debug, Clone)]
pub struct State {
    pub ships: Vec<Ship>,
    pub planets: Vec<Planet>,
    pub inputs: Inputs,
}

impl TryInto<State> for proto::State {
    type Error = ();

    fn try_into(self) -> Result<State, Self::Error> {
        let out = State {
            inputs: match self.inputs {
                Some(inputs) => inputs.try_into()?,
                None => return Err(()),
            },
            planets: self
                .planets
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<Planet>, ()>>()?,
            ships: self
                .ships
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<Ship>, ()>>()?,
        };

        Ok(out)
    }
}

impl From<State> for proto::State {
    fn from(state: State) -> proto::State {
        proto::State {
            inputs: Some(state.inputs.into()),
            planets: state.planets.into_iter().map(|p| p.into()).collect(),
            ships: state.ships.into_iter().map(|p| p.into()).collect(),
        }
    }
}
