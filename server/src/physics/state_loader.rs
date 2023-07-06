use std::{fs, process};

use common::data::state::State;

pub fn load_state(path: &str) -> State {
    let system_file = match fs::read_to_string(path) {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to read system file: {}", e);
            process::exit(1);
        }
    };
    
    let state: State = match ron::from_str(system_file.as_str()) {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to parse system file: {}", e);
            process::exit(1);
        }
    };

    state
}