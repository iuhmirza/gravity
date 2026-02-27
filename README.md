# Gravity 🌌

**Gravity** is a high-performance disk space analyzer built in Rust. It is designed to identify files and folders that exert a disproportionate "pull" on your storage—helping you find the bloat that traditional tools might miss.

Unlike simple `du` clones, Gravity aims to highlight folders that are heavy not just because of a single massive file, but because of cumulative "disproportionate" growth.

> [!WARNING]
> **Status: Work In Progress (WIP)** > This project is currently under active development. The CLI interface and core logic are subject to frequent changes.

---

## 🚀 Features

* **Dual Scanning Engines:** Includes both a high-concurrency **Asynchronous** scanner (powered by `tokio`) and a standard **Synchronous** scanner for performance comparison.
* **Smart Collection:** Implements multiple collection strategies (`HeapCollector` and `VecCollector`) to efficiently track the largest entries without consuming excessive memory.
* **Deep Insights:** Calculates total size and "size excluding the largest element" to help identify directories filled with many medium-sized files (true bloat).
* **Recursive Traversal:** Safely handles symlinks and directory structures.

---

## 📂 Project Structure

The project is organized into modular components:

* **`main.rs`**: The entry point that orchestrates the scan and handles the async runtime.
* **`scanner.rs`**: The core logic for filesystem traversal, implementing both `async` recursive tasks and `sync` loops.
* **`entry.rs`**: Defines the `Entry` data structure and the `EntryKind` enum. It handles the logic for comparing file "heaviness."
* **`collector.rs`**: A trait-based system for gathering results.
* **`HeapCollector`**: Uses a `BinaryHeap` to maintain the top $N$ largest items in $O(\log N)$ time.
* **`VecCollector`**: A simple vector-based collection for smaller datasets.



---

## 🛠️ Getting Started

### Prerequisites

* Rust (Stable)
* Cargo

### Installation

Clone the repository:

```bash
git clone https://github.com/iuhmirza/gravity.git
cd gravity

```

### Running the Project

Currently, the project is configured to scan the `/home` directory by default. You can run the built-in benchmarks and scanners using:

```bash
cargo run

```

### Running Tests

Gravity includes tests to compare the accuracy and performance of the different scanning and collection methods:

```bash
cargo test

```

---

## 📊 How it Works

Gravity doesn't just look at the `total_size`. It focuses on the **size_excluding_max**.

By subtracting the largest single element from a directory's total weight, Gravity identifies folders that are cluttered with numerous files—often a better indicator of "garbage" or cache build-up than a single large ISO or video file.

---

## 🏗️ Roadmap

* [ ] Add a proper CLI interface using `clap`.
* [ ] Implement customizable depth limits.
* [ ] Add more sophisticated "gravity" metrics (weighted bloat scores).
* [ ] Visual TUI for navigating the filesystem.

**Author:** [iuhmirza](https://github.com/iuhmirza)

