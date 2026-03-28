# 🏎️ Votrexian Database (VODB)

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)

An ultra-fast, in-memory Key-Value DBMS written in **Rust**. Built for high-performance data storage in the **Votrexian** ecosystem, featuring native LZ4 compression and binary snapshot (.vodb).

## 🧠 Background: Why VODB?

"If you can only need processor and and Memory for database, why would you need Physical Storage? 🗿🗿"

— _The VODB Manifesto_

## 🔥 Core Capabilities

- **In-Memory Speed**: Leveraging Rust's ownership for zero-cost data handling.
- **LZ4 Real-time Compression**: Ultra-fast compression/decompression to save RAM.
- **VODB Snapshots**: Persistence system using Binary Serialization (Bincode).
- **Big Number IDs**: Support for massive IDs (up to 512-bit+) via `num-bigint`.
- **Validation Guardrails**: Strict 512 MB per-content limit for system stability.

## 🛠️ Tech Stack

- **Core**: Rust (1.90+)
- **Serialization**: Bincode, Serde, Serde_JSON
- **Compression**: lz4_flex
- **Math**: num-bigint

## 🚀 Getting Started

1. **Clone & Build**:

   ```bash
   git clone [https://github.com/ATmm4869/votrexian_db.git](https://github.com/ATom4869/votrexian_db.git)
   cargo build --release
   ```

2. **Clone & Build**:

   ```bash
   ./target/release/votrexian_db
   ```
