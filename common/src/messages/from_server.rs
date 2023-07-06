use serde::{Serialize, Deserialize};

use crate::data::state::State;

#[derive(Serialize, Deserialize)]
pub enum FromServerMessage {
    Update(State),
}