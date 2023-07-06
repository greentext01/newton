use serde::{Serialize, Deserialize};

pub struct BincodeEncoder {
    output_buffer: Vec<u8>,
}

impl BincodeEncoder {
    pub fn new() -> BincodeEncoder {
        BincodeEncoder {
            output_buffer: Vec::new(),
        }
    }

    pub fn encode<M: Serialize>(&mut self, message: M) -> &[u8] {
        self.output_buffer.clear();
        bincode::serialize_into(&mut self.output_buffer, &message).unwrap();
        &self.output_buffer
    }

    pub fn decode<'a, M: Deserialize<'a>>(message_bin: &'a [u8]) -> Option<M> {
        bincode::deserialize::<M>(message_bin).ok()
    }
}
