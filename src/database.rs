use crate::models::{DataMetadata, RawInput};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

const MAX_CONTENT_SIZE: usize = 512 * 1024 * 1024;

pub struct BlobberDB {
    pub storage: HashMap<BigUint, DataMetadata>,
}

impl BlobberDB {
    /// Inisialisasi Database baru di RAM
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert_bulk(&mut self, raw_data: Vec<RawInput>) {
        for item in raw_data {
            // --- PROSES PARSING ID ---
            let id_biguint =
                BigUint::parse_bytes(item.data_id.as_bytes(), 10).unwrap_or_else(|| {
                    eprintln!("❌ ID {} bukan format angka valid, pakai 0.", item.data_id);
                    BigUint::from(0u32)
                });

            if item.data_content.len() > MAX_CONTENT_SIZE {
                eprintln!("⚠️ SKIPPED: ID {} terlalu besar.", id_biguint);
                continue;
            }

            let compressed = compress_prepend_size(item.data_content.as_bytes());

            let meta = DataMetadata {
                data_id: id_biguint.clone(), // Pakai hasil parsing tadi
                data_name: item.data_name,
                data_is_encrypted: item.data_is_encrypted,
                data_content: compressed,
            };

            self.storage.insert(id_biguint, meta);
        }
        println!("✅ Bulk import selesai diproses.");
    }

    pub fn get_data(&self, id: &BigUint) -> Option<(DataMetadata, String)> {
        if let Some(meta) = self.storage.get(id) {
            // Dekompresi balik ke original bytes
            match decompress_size_prepended(&meta.data_content) {
                Ok(decompressed) => {
                    let original_text = String::from_utf8_lossy(&decompressed).to_string();
                    Some((meta.clone(), original_text))
                }
                Err(e) => {
                    eprintln!("❌ Error dekompresi ID {}: {}", id, e);
                    None
                }
            }
        } else {
            None
        }
    }

    /// List semua file tanpa dekompresi (Responsif dan cepat)
    pub fn list_metadata(&self) {
        if self.storage.is_empty() {
            println!("📭 Database kosong.");
            return;
        }

        println!("{:-<50}", "");
        println!(
            "{:<15} | {:<20} | {:<10}",
            "ID", "NAMA FILE", "UKURAN (LZ4)"
        );
        println!("{:-<50}", "");

        for meta in self.storage.values() {
            println!(
                "{:<15} | {:<20} | {:>7} KB",
                meta.data_id,
                meta.data_name,
                meta.data_content.len() / 1024
            );
        }
        println!("{:-<50}", "");
    }

    /// Option A: Simpan Snapshot DB dari RAM ke Disk (Save)
    pub fn save_snapshot(&self, path: &str) {
        println!("💾 Menyimpan snapshot ke {}...", path);
        let mut file = File::create(path).expect("Gagal membuat file database");

        // Serialize HashMap ke Biner menggunakan Bincode
        let encoded: Vec<u8> =
            bincode::serialize(&self.storage).expect("Gagal melakukan serialisasi data");

        file.write_all(&encoded).expect("Gagal menulis ke disk");
        println!("✨ Snapshot aman di disk!");
    }

    /// Option A: Muat Snapshot DB dariDisk ke RAM (Restore)
    pub fn load_snapshot(&mut self, path: &str) {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                println!("ℹ️ File snapshot tidak ditemukan, memulai DB baru.");
                return;
            }
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Gagal membaca file");

        // Kembalikan biner menjadi struktur data Rust
        self.storage =
            bincode::deserialize(&buffer).expect("Format file database tidak dikenali atau korup");

        println!(
            "🚀 Firing on! {} records berhasil dimuat ke RAM.",
            self.storage.len()
        );
    }

    pub fn search_by_name(&self, query: &str) {
        let query_lower = query.to_lowercase();
        let mut found_count = 0;

        println!("\n🔍 Hasil pencarian untuk: '{}'", query);
        println!("{:-<60}", "");
        println!("{:<15} | {:<30} | {:<10}", "ID", "NAMA FILE", "UKURAN");
        println!("{:-<60}", "");

        for meta in self.storage.values() {
            if meta.data_name.to_lowercase().contains(&query_lower) {
                println!(
                    "{:<15} | {:<30} | {:>7} KB",
                    meta.data_id,
                    meta.data_name,
                    meta.data_content.len() / 1024
                );
                found_count += 1;
            }
        }

        if found_count == 0 {
            println!("❌ Tidak ada file yang cocok dengan kata kunci tersebut.");
        } else {
            println!("{:-<60}", "");
            println!("✅ Ditemukan {} file.", found_count);
        }
    }
}
