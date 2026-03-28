import json
import random
import base64

def to_b64(content):
    """Helper buat ubah data jadi Base64 string agar aman dikirim lewat JSON."""
    if isinstance(content, str):
        return base64.b64encode(content.encode('utf-8')).decode('utf-8')
    return base64.b64encode(content).decode('utf-8')

def generate_varied_data(multiplier=1):
    data_list = []
    # Base ID raksasa untuk ngetes BigUint Rust
    base_id = 2 * (10**20)

    # Definisi 10 Tipe Data dengan Label Tipe-nya
    def get_templates(i):
        return [
            # (Nama File, Label Tipe, Konten)
            ("welcome_msg.txt", "TEXT", f"User_{i}: Welcome to Votrexian!"),
            ("documentation.md", "MARKDOWN", "# Project BlobberX\n" + "Lorem Ipsum " * 50),
            ("config_prod.json", "JSON", json.dumps({"db": "vodb", "port": 7100, "active": True})),
            ("sensor_metrics.csv", "CSV", f"timestamp,temp,volt\n2026-03-28,32.5,220"),
            ("firmware_blob.bin", "BINARY", random.randbytes(512)),
            ("system_auth.log", "LOG", f"2026-03-28 [AUTH] Login success for user_id_{i}"),
            ("main_logic.rs", "RUST", "fn main() { println!(\"Votrexian Engine Running\"); }"),
            ("encrypted_vault.key", "ENCRYPTED", "HEX_7F82A1B2C3D4E5F6"),
            ("user_preferences.bin", "BSON", json.dumps([True, False, True, i]).encode()),
            ("engine_report.pdf", "PDF", b"%PDF-1.4 Mock Binary Content " + random.randbytes(100))
        ]

    total_count = 0
    for m in range(multiplier):
        templates = get_templates(m)
        for idx, (name, d_type, content) in enumerate(templates):
            # ID unik: base_id + (multiplier * 10) + index_template
            current_id = base_id + (m * 10) + idx
            
            entry = {
                "data_id": str(current_id),
                "data_name": f"{m:03}_{name}", # Tambah prefix biar gampang disort nama
                "data_type": d_type,           # <-- FIELD BARU
                "data_is_encrypted": d_type == "ENCRYPTED",
                "data_content": to_b64(content)
            }
            data_list.append(entry)
            total_count += 1
            
    filename = 'varied_test_data.json'
    with open(filename, 'w') as f:
        json.dump(data_list, f, indent=2)
    
    print(f"✅ Berhasil generate {total_count} data!")
    print(f"📂 File: {filename}")
    print(f"multiplier: {multiplier} set x 10 tipe")

if __name__ == "__main__":
    # Mas Arson mau berapa banyak? 
    # multiplier=100 artinya 1000 data
    X = 10 
    generate_varied_data(multiplier=X)