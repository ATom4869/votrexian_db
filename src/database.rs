use crate::models::{DataMetadata, RawInput};
use base64::{Engine as _, engine::general_purpose};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

const MAX_CONTENT_SIZE: usize = 512 * 1024 * 1024;

pub struct DBMonitoring {
    pub storage: HashMap<BigUint, DataMetadata>,
    pub current_usage: u64,
    pub max_capacity: u64,
}

impl DBMonitoring {
    /// Inisialisasi Database baru di RAM
    pub fn new() -> Self {
        // Limit 60% dari RAM
        let limit_gb = 3.6;
        let max_bytes = (limit_gb * 1024.0 * 1024.0 * 1024.0) as u64;

        DBMonitoring {
            storage: HashMap::new(),
            current_usage: 0,
            max_capacity: max_bytes,
        }
    }

    /// Ambil statistik pemakaian RAM untuk Dashboard
    pub fn get_stats(&self) -> (f64, f64, f64) {
        let usage_mb = self.current_usage as f64 / 1024.0 / 1024.0;
        let limit_mb = self.max_capacity as f64 / 1024.0 / 1024.0;
        let percent = (usage_mb / limit_mb) * 100.0;
        (usage_mb, limit_mb, percent)
    }

    pub fn insert_bulk(&mut self, raw_data: Vec<RawInput>) {
        for item in raw_data {
            // 1. Parsing ID
            let id_biguint = BigUint::parse_bytes(item.data_id.trim().as_bytes(), 10)
                .unwrap_or_else(|| {
                    eprintln!("❌ ID '{}' tidak valid, skip.", item.data_id);
                    BigUint::from(0u32)
                });

            // 2. Cek Limit Per-File (512MB)
            if item.data_content.len() > MAX_CONTENT_SIZE {
                eprintln!("⚠️ SKIPPED: ID {} terlalu besar (Limit 512MB).", id_biguint);
                continue;
            }

            // 3. Decode Base64 (Mencegah pembengkakan 33% dari JSON)
            let raw_binary = match general_purpose::STANDARD.decode(item.data_content.trim()) {
                Ok(bytes) => bytes,
                Err(_) => {
                    // Kalau gagal decode, simpan string aslinya sebagai byte (Fallback)
                    item.data_content.into_bytes()
                }
            };

            // 4. Kompresi LZ4
            let compressed = compress_prepend_size(&raw_binary);
            let entry_size = compressed.len() as u64;

            // 5. RAM Guard (Check 60% Limit)
            if self.current_usage + entry_size > self.max_capacity {
                eprintln!("🚨 ALERT: Database Full! Gagal simpan ID {}.", id_biguint);
                continue;
            }

            let meta = DataMetadata {
                data_id: id_biguint.clone(),
                data_name: item.data_name,
                data_type: item.data_type,
                data_is_encrypted: item.data_is_encrypted,
                data_content: compressed,
            };

            // Update storage & usage tracker (handle overwrite)
            if let Some(old_data) = self.storage.insert(id_biguint.clone(), meta) {
                self.current_usage -= old_data.data_content.len() as u64;
                println!("⚠️ OVERWRITE: ID {} diperbarui.", id_biguint);
            } else {
                println!("✅ INSERT: {} (ID: {})", item.data_id, id_biguint);
            }
            self.current_usage += entry_size;
        }
        println!("✅ Bulk import selesai.");
    }

    pub fn save_snapshot(&self, path: &str) {
        let encoded: Vec<u8> = bincode::serialize(&self.storage).expect("Gagal serialisasi");
        let mut file = File::create(path).expect("Gagal membuat file");
        file.write_all(&encoded).expect("Gagal menulis data");
        println!("💾 Snapshot tersimpan di: {}", path);
    }

    pub fn load_snapshot(&mut self, path: &str) {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return,
        };
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Gagal baca file");

        self.storage = bincode::deserialize(&buffer).expect("Format file korup");

        // FIX: Hitung ulang usage RAM setelah load
        self.current_usage = self
            .storage
            .values()
            .map(|m| m.data_content.len() as u64)
            .sum();

        println!(
            "🚀 Firing on! {} records ({:.2} MB) dimuat.",
            self.storage.len(),
            self.current_usage as f64 / 1024.0 / 1024.0
        );
    }

    pub fn list_metadata(&self) {
        if self.storage.is_empty() {
            println!("📭 Database kosong.");
            return;
        }
        let mut entries: Vec<&DataMetadata> = self.storage.values().collect();
        entries.sort_by(|a, b| a.data_id.cmp(&b.data_id));

        let sep = "=".repeat(95);
        println!(
            "\n{}\n{:<22} | {:<30} | {:<15} | {:<15}\n{}",
            sep,
            "ID",
            "NAMA FILE",
            "TIPE",
            "UKURAN (LZ4)",
            "-".repeat(95)
        );

        for meta in entries {
            println!(
                "{:<22} | {:<30} | {:<15} | {:>10.2} KB",
                meta.data_id.to_string(),
                if meta.data_name.len() > 28 {
                    format!("{}...", &meta.data_name[..25])
                } else {
                    meta.data_name.clone()
                },
                meta.data_type,
                meta.data_content.len() as f64 / 1024.0
            );
        }
        println!("{}", sep);
    }

    pub fn search_by_name(&self, query: &str) {
        let mut res: Vec<&DataMetadata> = self
            .storage
            .values()
            .filter(|m| m.data_name.to_lowercase().contains(&query.to_lowercase()))
            .collect();
        res.sort_by(|a, b| a.data_id.cmp(&b.data_id));
        self.print_table("Hasil Cari Nama", &res);
    }

    pub fn search_by_type(&self, q_type: &str) {
        let mut res: Vec<&DataMetadata> = self
            .storage
            .values()
            .filter(|m| m.data_type.to_lowercase() == q_type.to_lowercase())
            .collect();
        res.sort_by(|a, b| a.data_id.cmp(&b.data_id));
        self.print_table(&format!("Kategori: {}", q_type), &res);
    }

    fn print_table(&self, title: &str, items: &Vec<&DataMetadata>) {
        if items.is_empty() {
            println!("❌ Data tidak ditemukan.");
            return;
        }
        let sep = "=".repeat(95);
        println!(
            "\n🔍 {}\n{}\n{:<22} | {:<30} | {:<15} | {:<15}\n{}",
            title,
            sep,
            "ID",
            "NAMA FILE",
            "TIPE",
            "UKURAN (LZ4)",
            "-".repeat(95)
        );
        for m in items {
            println!(
                "{:<22} | {:<30} | {:<15} | {:>10.2} KB",
                m.data_id.to_string(),
                if m.data_name.len() > 28 {
                    format!("{}...", &m.data_name[..25])
                } else {
                    m.data_name.clone()
                },
                m.data_type,
                m.data_content.len() as f64 / 1024.0
            );
        }
        println!("{}\n✅ Total: {} file.\n", sep, items.len());
    }

    pub fn delete_data(&mut self, id: &BigUint) -> bool {
        if let Some(removed) = self.storage.remove(id) {
            self.current_usage -= removed.data_content.len() as u64;
            println!(
                "🗑️ ID {} dihapus. RAM lega {:.2} KB.",
                id,
                removed.data_content.len() as f64 / 1024.0
            );
            return true;
        }
        println!("❌ ID tidak ditemukan.");
        false
    }

    pub fn view_content(&self, id: &BigUint) {
        if let Some(meta) = self.storage.get(id) {
            let decompressed =
                decompress_size_prepended(&meta.data_content).expect("Gagal dekompresi");
            println!("\n📄 File: {} | Tipe: {}", meta.data_name, meta.data_type);
            println!("--------------------------------------------------");
            println!("{}", String::from_utf8_lossy(&decompressed));
            println!("--------------------------------------------------");
        } else {
            println!("❌ Data ID {} tidak ditemukan.", id);
        }
    }
}
