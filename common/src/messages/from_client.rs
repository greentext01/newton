use serde::{Serialize, Deserialize};

use crate::data::inputs::{NavMode};

#[derive(Debug, Serialize, Deserialize)]
pub enum FromClientMessage {
    NavMode(NavMode),
    Throttle(f32),
}
