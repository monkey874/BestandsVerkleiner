use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BytesMap {
    pub bit_length: usize,
    pub bytes: Vec<u8>,
}