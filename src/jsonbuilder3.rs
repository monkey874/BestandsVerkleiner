use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HuffFile {
    pub bit_length: usize,
    pub data: Vec<u8>,
}