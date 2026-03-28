use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawInput {
    pub data_id: String,
    pub data_name: String,
    pub data_type: String, // Label tipe data (TEXT, PDF, dll)
    pub data_is_encrypted: bool,
    pub data_content: String, // Masih dalam bentuk Base64 String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataMetadata {
    pub data_id: BigUint, // BigInt biar kuat nampung ID blockchain-mu
    pub data_name: String,
    pub data_type: String,
    pub data_is_encrypted: bool,
    pub data_content: Vec<u8>, // Sudah jadi Vector Byte & Terkompresi LZ4
}
