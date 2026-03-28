import json
import random
import base64

def generate_test_data(count=1000, start_id=None):
    data_list = []
    
    # Jika start_id tidak ditentukan, default pake kepala 1 (10^20)
    # Ini biar beda jalur sama varied_data yang pake kepala 2
    base_id = start_id if start_id else 10**20 
    
    print(f"🚀 Memulai generate {count} data standard...")

    for i in range(1, count + 1):
        # ID unik agar tidak overwrite di HashMap Rust
        current_id = base_id + i 
        
        # Konten teks dummy
        original_text = f"Ini adalah konten dummy Votrexian nomor urut {i}. " * 30
        
        # WAJIB: Encode ke Base64 (Syarat mutlak Engine Rust kita sekarang)
        b64_content = base64.b64encode(original_text.encode('utf-8')).decode('utf-8')
        
        entry = {
            "data_id": str(current_id),
            "data_name": f"votrexian_file_{i:04}.txt",
            "data_type": "TEXT", # Field baru agar sinkron dengan struct Rust
            "data_is_encrypted": random.choice([True, False]),
            "data_content": b64_content
        }
        data_list.append(entry)
    
    output_file = 'test_data.json'
    with open(output_file, 'w', encoding='utf-8') as f:
        # Pake indent=2 biar enak dibaca kalau dibuka manual
        json.dump(data_list, f, indent=2)
    
    print(f"✅ Berhasil generate {count} data ke {output_file}")
    print(f"💡 Range ID: {base_id + 1} s/d {base_id + count}")

if __name__ == "__main__":
    # Mas Arson bisa ganti jumlahnya di sini
    generate_test_data(1000)