use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawInput {
    pub data_id: String,
    pub data_name: String,
    pub data_type: String,
    pub data_is_encrypted: bool,
    pub data_content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataMetadata {
    pub data_id: BigUint,
    pub data_name: String,
    pub data_type: String,
    pub data_is_encrypted: bool,
    pub data_content: Vec<u8>,
}
