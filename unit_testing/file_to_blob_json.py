import json
import os
import base64
import sys
from tqdm import tqdm

BUFFER_SIZE = 1 * 1024 * 1024 # 1MB Buffer buat progress bar

def process_single_file(filepath, data_id):
    if not os.path.exists(filepath):
        print(f"❌ Skip: File '{filepath}' tidak ada.")
        return None

    file_size = os.path.getsize(filepath)
    ext = os.path.splitext(filepath)[1].lower().replace(".", "")
    data_type = ext.upper() if ext else "RAW"

    file_bytes = bytearray()
    
    # Progress Bar per File
    with open(filepath, 'rb') as f:
        with tqdm(total=file_size, unit='B', unit_scale=True, desc=f"📦 Packing {os.path.basename(filepath)}") as pbar:
            while True:
                chunk = f.read(BUFFER_SIZE)
                if not chunk:
                    break
                file_bytes.extend(chunk)
                pbar.update(len(chunk))

    # Konversi ke Base64
    encoded_string = base64.b64encode(file_bytes).decode('utf-8')
    
    return {
        "data_id": str(data_id),
        "data_name": os.path.basename(filepath),
        "data_type": data_type,
        "data_is_encrypted": False,
        "data_content": encoded_string
    }

def main():
    # Perintah: python script.py output.json 7001 file1 file2...
    if len(sys.argv) < 4:
        print("💡 Usage: python3 file_to_blob_json.py <output_json> <start_id> <file1> <file2> ...")
        return

    output_path = sys.argv[1]
    try:
        start_id = int(sys.argv[2])
    except ValueError:
        print("❌ Error: Start ID harus angka!")
        return
        
    files_to_process = sys.argv[3:]
    payload_list = []
    
    print(f"🚀 Votrexian Blobber: Mengonversi {len(files_to_process)} file...")

    for i, filepath in enumerate(files_to_process):
        current_id = start_id + i
        # Bersihkan path dari tanda petik terminal
        clean_path = filepath.strip("'").strip('"')
        
        entry = process_single_file(clean_path, current_id)
        if entry:
            payload_list.append(entry)

    if payload_list:
        print(f"💾 Menulis hasil ke {output_path}...")
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(payload_list, f)
        
        final_size = os.path.getsize(output_path) / (1024*1024)
        print(f"✅ Selesai! JSON Blob siap: {final_size:.2f} MB")
    else:
        print("❌ Gagal: Tidak ada data yang berhasil dikonversi.")

if __name__ == "__main__":
    main()