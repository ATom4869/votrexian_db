import json
import os
import base64
import sys
from tqdm import tqdm

BUFFER_SIZE = 1 * 1024 * 1024 
LIMIT_512MB = 512 * 1024 * 1024

def process_single_file(filepath, data_id):
    if not os.path.exists(filepath):
        print(f"❌ Skip: File {filepath} tidak ada.")
        return None

    file_size = os.path.getsize(filepath)
    if file_size > LIMIT_512MB:
        print(f"⚠️ Skip: {filepath} kegedean ({file_size / (1024*1024):.2f} MB).")
        return None

    file_bytes = bytearray()
    with open(filepath, 'rb') as f:
        with tqdm(total=file_size, unit='B', unit_scale=True, desc=f"Reading {os.path.basename(filepath)}") as pbar:
            while True:
                data = f.read(BUFFER_SIZE)
                if not data: break
                file_bytes.extend(data)
                pbar.update(len(data))

    encoded_string = base64.b64encode(file_bytes).decode('utf-8')
    
    return {
        "data_id": str(data_id),
        "data_name": os.path.basename(filepath),
        "data_is_encrypted": False,
        "data_content": encoded_string
    }

def main():
    # Contoh penggunaan: python3 script.py output.json 1001 file1.zip file2.jpg file3.pdf
    if len(sys.argv) < 4:
        print("Usage: python3 script.py <output_json> <start_id> <file1> <file2> <file3> ...")
        return

    output_path = sys.argv[1]
    start_id = int(sys.argv[2])
    files_to_process = sys.argv[3:]

    payload_list = []
    
    print(f"🚀 Memulai konversi {len(files_to_process)} file ke satu JSON...")

    for i, filepath in enumerate(files_to_process):
        current_id = start_id + i
        data_entry = process_single_file(filepath, current_id)
        if data_entry:
            payload_list.append(data_entry)

    if not payload_list:
        print("❌ Tidak ada data yang berhasil dikonversi.")
        return

    print(f"💾 Menulis {len(payload_list)} data ke {output_path}...")
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(payload_list, f)

    print(f"✅ Selesai! Total size JSON: {os.path.getsize(output_path) / (1024*1024):.2f} MB")

if __name__ == "__main__":
    main()