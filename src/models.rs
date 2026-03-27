use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

/// 1. DataMetadata: Struktur yang HIDUP di RAM & DISK (.vodb)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataMetadata {
    pub data_id: BigUint,
    pub data_name: String,
    pub data_is_encrypted: bool,
    pub data_content: Vec<u8>, // Simpan byte terkompresi di sini
}

/// 2. RawInput: Struktur khusus untuk jembatan INPUT JSON
#[derive(Deserialize, Debug)]
pub struct RawInput {
    pub data_id: String, // Kita terima sebagai String dulu dari JSON
    pub data_name: String,
    pub data_is_encrypted: bool,
    pub data_content: String,
}
