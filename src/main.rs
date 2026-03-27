mod database;
mod models;

use database::BlobberDB;
use num_bigint::BigUint;
use std::io::{self, Write};

fn format_vodb_name(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return "votrexian.vodb".to_string();
    }

    if trimmed.ends_with(".vodb") {
        trimmed.to_string()
    } else {
        format!("{}.vodb", trimmed)
    }
}

fn main() {
    let mut db = BlobberDB::new();

    // State: Menyimpan path DB yang sedang aktif
    let mut current_db_path = "votrexian.vodb".to_string();

    println!("--- 🟢 Votrexian DBMS Engine Active ---");

    // Auto-Load saat pertama kali startup (Firing On)
    db.load_snapshot(&current_db_path);

    loop {
        // Tampilkan info DB mana yang sedang dikerjakan
        println!("\n📂 Active Database: [{}]", current_db_path);
        print!(
            " [1] Bulk Import\n [2] List All\n [3] View Content\n [4] Search Name\n [5] Save As\n [6] Load\n [7] Exit\n> "
        );
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("📄 Masukkan Path File JSON (ex: ./data.json): ");
                io::stdout().flush().unwrap();
                let mut path_input = String::new();
                io::stdin().read_line(&mut path_input).unwrap();
                let path = path_input.trim();

                // Baca file dan parse
                match std::fs::read_to_string(path) {
                    Ok(json_content) => {
                        match serde_json::from_str::<Vec<models::RawInput>>(&json_content) {
                            Ok(raw_list) => {
                                println!("🚀 Mengimpor {} data...", raw_list.len());
                                db.insert_bulk(raw_list);
                            }
                            Err(e) => eprintln!("❌ Error parsing JSON: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Gagal baca file di {}: {}", path, e),
                }
            }

            "2" => {
                db.list_metadata();
            }

            "3" => {
                print!("🔍 Masukkan ID Data (Angka): ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();

                if let Some(id) = BigUint::parse_bytes(id_input.trim().as_bytes(), 10) {
                    if let Some((meta, content)) = db.get_data(&id) {
                        println!("\n--- 📄 DATA FOUND ---");
                        println!("ID       : {}", meta.data_id);
                        println!("Name     : {}", meta.data_name);
                        println!("Enc      : {}", meta.data_is_encrypted);
                        println!("Size     : {} bytes (Compressed)", meta.data_content.len());
                        println!("Content  :\n{}", content);
                        println!("---------------------");
                    } else {
                        println!("❌ ID {} tidak ditemukan di RAM.", id);
                    }
                } else {
                    println!("❌ Format ID salah! Harus angka.");
                }
            }

            "4" => {
                print!("🔍 Ketik Nama File/Kata Kunci: ");
                io::stdout().flush().unwrap();
                let mut query = String::new();
                io::stdin().read_line(&mut query).unwrap();

                db.search_by_name(query.trim());
            }

            "5" => {
                print!("💾 Simpan Sebagai (Nama file saja): ");
                io::stdout().flush().unwrap();
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).unwrap();

                // Update state path dan simpan
                current_db_path = format_vodb_name(&file_name);
                db.save_snapshot(&current_db_path);
            }

            "6" => {
                print!("📂 Nama snapshot yang mau di-load: ");
                io::stdout().flush().unwrap();
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).unwrap();

                // Ganti fokus ke file baru dan load
                current_db_path = format_vodb_name(&file_name);
                db.load_snapshot(&current_db_path);
            }

            "7" => {
                println!(
                    "💾 Menjalankan Auto-Save terakhir ke {}...",
                    current_db_path
                );
                db.save_snapshot(&current_db_path);
                println!("👋 Sayonara, Mas Arson! DBMS Shutdown.");
                break;
            }

            _ => println!("❓ Pilihan tidak valid."),
        }
    }
}
