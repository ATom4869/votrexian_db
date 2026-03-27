import json
import random

def generate_test_data(count=1000):
    data_list = []
    for i in range(1, count + 1):
        # Generate ID gede ala-ala blockchain/big number
        big_id = 10**20 + i 
        
        entry = {
            "data_id": str(big_id), # Serde BigUint bisa terima string angka
            "data_name": f"votrexian_file_{i:03}.txt",
            "data_is_encrypted": random.choice([True, False]),
            "data_content": f"Ini adalah konten dummy untuk file nomor {i}. " * 50 # Biar rada panjang dikit
        }
        data_list.append(entry)
    
    with open('test_data.json', 'w') as f:
        json.dump(data_list, f, indent=2)
    
    print(f"✅ Berhasil generate {count} data ke test_data.json")

if __name__ == "__main__":
    generate_test_data(1000)