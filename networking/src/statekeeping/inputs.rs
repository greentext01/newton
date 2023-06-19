use num_derive::{ToPrimitive, FromPrimitive};
use num_traits::FromPrimitive;

use crate::proto::state::{Inputs as NetInputs};

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum NavMode {
    MAN,
}

#[derive(Debug, Clone)]
pub struct Inputs {
    pub navmode: NavMode,
    pub throttle: f32,
}

impl TryInto<Inputs> for NetInputs {
    type Error = ();

    fn try_into(self) -> Result<Inputs, Self::Error> {
        let out = Inputs {
            navmode: FromPrimitive::from_i32(self.navmode).unwrap_or(NavMode::MAN),
            throttle: self.throttle,
        };

        Ok(out)
    }
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
