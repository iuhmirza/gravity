# Gravity 🌌

**Gravity** is a work-in-progress disk usage analyzer written in Rust.
Its goal is simple: **find the files and folders that take up the most space on your drive**.

Gravity recursively scans directories, calculates file and directory sizes, and surfaces the largest space consumers using a priority queue.

---

## ✨ Features (Current)

* 🚀 Asynchronous directory scanning with `tokio`
* 📁 Recursive traversal of directories
* 📊 Size aggregation for files and directories
* 🧠 Uses a `BinaryHeap` to track largest entries
* 🔝 Displays the top 10 largest entries found

---

## 📦 How It Works

Gravity scans a directory tree (currently hardcoded to `/home`) and:

1. Walks through all subdirectories.
2. Collects file sizes.
3. Aggregates total directory sizes.
4. Pushes results into a `BinaryHeap`.
5. Prints the 10 largest entries.

### Directory Size Logic

Each directory stores:

* `total_size` — total size of all contents
* `size_excluding_max` — total size minus its largest child
  (used for smarter ordering in the heap)

Entries are ordered by:

1. `size_excluding_max`
2. `total_size`
3. `path`

This allows large directories with many moderately large files to surface properly, instead of being dominated by a single huge file.

---

## 🧠 Project Structure

```
src/
├── main.rs        # Entry point
├── scanner.rs     # Recursive async directory scanning
├── entry.rs       # Entry struct + ordering logic
└── collector.rs   # (Planned / placeholder)
```

---

## 🛠️ Installation

Make sure you have Rust installed:

```bash
rustup install stable
```

Clone the repository:

```bash
git clone https://github.com/yourusername/gravity.git
cd gravity
```

Run:

```bash
cargo run --release
```

---

## 📄 Example Output

```
Scanning
1: Entry { path: "...", kind: Directory, total_size: ..., size_excluding_max: ... }
2: Entry { path: "...", kind: File, total_size: ..., size_excluding_max: ... }
...
```

---

## 🔍 Current Limitations

* Path is hardcoded
* No CLI arguments
* No filtering options
* No progress reporting
* No output formatting (raw debug print only)
* No testing
* No symlink handling beyond skipping

---

## 🗺️ Roadmap / TODO

### Core Improvements

* [ ] Add CLI argument parsing (scan path, top N results)
* [ ] Add human-readable size formatting (KB, MB, GB, TB)
* [ ] Improve error handling and reporting
* [ ] Add depth limiting
* [ ] Add ignore patterns (e.g., `.git`, `node_modules`)
* [ ] Improve performance for extremely large directory trees

### UX Improvements

* [ ] Pretty terminal output (tables or colors)
* [ ] Add progress indicator
* [ ] Add interactive mode
* [ ] Export results to JSON/CSV

### Architecture

* [ ] Improve collector design (reduce locking contention)
* [ ] Add configuration struct
* [ ] Add benchmarking suite
* [ ] Write unit tests
* [ ] Add integration tests

### Cross-Platform

* [ ] Windows support testing
* [ ] macOS testing
* [ ] Handle platform-specific filesystem quirks

---

## 🧪 Why "Gravity"?

Because large files and directories exert **gravitational pull** on your storage.

Gravity helps you find the black holes on your drive before they consume everything.

---

## 🤝 Contributing

This project is in early development and open to ideas, refactors, and improvements.

If you'd like to contribute:

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

---

## 📜 License

TBD

---

**Gravity** — Find what’s weighing your storage down.
