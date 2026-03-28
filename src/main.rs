mod database;
mod models;

use database::DBMonitoring;
use num_bigint::BigUint;
use std::fs;
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
    let mut db = DBMonitoring::new();
    let mut current_db_path = "votrexian.vodb".to_string();

    println!("--- 🟢 Votrexian DBMS Engine Active ---");
    println!("Dashboard rapih, hati tenang ya Mas Arson. ✨");

    db.load_snapshot(&current_db_path);

    loop {
        let (used, limit, pct) = db.get_stats();
        println!("\n📂 Active DB: [{}]", current_db_path);
        println!(
            "📊 RAM Usage: {:.2} MB / {:.2} MB ({:.1}%)",
            used, limit, pct
        );

        if pct > 90.0 {
            println!("⚠️ WARNING: Memory almost full!");
        }

        println!("--------------------------------------------------");
        println!(" [1] Bulk Import (JSON)");
        println!(" [2] List All Data (Sorted)");
        println!(" [3] View Data Content");
        println!(" [4] Search by Name");
        println!(" [5] Search by Type");
        println!(" [6] Delete Data (ID)");
        println!(" [7] Save Snapshot");
        println!(" [8] Load Another Snapshot");
        println!(" [9] Exit & Auto-Save");
        print!("--------------------------------------------------\n> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("📂 Path file JSON: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                if let Ok(content) = fs::read_to_string(path.trim()) {
                    let raw_data: Vec<models::RawInput> =
                        serde_json::from_str(&content).expect("Format JSON salah!");
                    db.insert_bulk(raw_data);
                } else {
                    println!("❌ File tidak ditemukan!");
                }
            }
            "2" => db.list_metadata(),
            "3" => {
                print!("🔍 Masukkan ID: ");
                io::stdout().flush().unwrap();
                let mut id_i = String::new();
                io::stdin().read_line(&mut id_i).unwrap();
                if let Some(id) = BigUint::parse_bytes(id_i.trim().as_bytes(), 10) {
                    db.view_content(&id);
                } else {
                    println!("❌ ID salah!");
                }
            }
            "4" => {
                print!("🔍 Nama File: ");
                io::stdout().flush().unwrap();
                let mut q = String::new();
                io::stdin().read_line(&mut q).unwrap();
                db.search_by_name(q.trim());
            }
            "5" => {
                print!("📂 Tipe (TEXT/PDF/dll): ");
                io::stdout().flush().unwrap();
                let mut t = String::new();
                io::stdin().read_line(&mut t).unwrap();
                db.search_by_type(t.trim());
            }
            "6" => {
                print!("🗑️ ID yang mau dihapus: ");
                io::stdout().flush().unwrap();
                let mut id_i = String::new();
                io::stdin().read_line(&mut id_i).unwrap();
                if let Some(id) = BigUint::parse_bytes(id_i.trim().as_bytes(), 10) {
                    db.delete_data(&id);
                } else {
                    println!("❌ ID salah!");
                }
            }
            "7" => {
                print!("💾 Nama Snapshot: ");
                io::stdout().flush().unwrap();
                let mut f = String::new();
                io::stdin().read_line(&mut f).unwrap();
                current_db_path = format_vodb_name(&f);
                db.save_snapshot(&current_db_path);
            }
            "8" => {
                print!("📂 Load Snapshot: ");
                io::stdout().flush().unwrap();
                let mut f = String::new();
                io::stdin().read_line(&mut f).unwrap();
                current_db_path = format_vodb_name(&f);
                db.load_snapshot(&current_db_path);
            }
            "9" => {
                println!("💾 Auto-Saving to {}...", current_db_path);
                db.save_snapshot(&current_db_path);
                println!("👋 Sayonara, Engine Shutdown. 🚀");
                break;
            }
            _ => println!("❓ Pilihan tidak ada."),
        }
    }
}
