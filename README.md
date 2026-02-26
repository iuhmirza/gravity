Here is an updated **README.md** with a dedicated **Performance Considerations & Optimization Opportunities** section added.

---

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
git clone https://github.com/iuhmirza/gravity.git
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

# ⚡ Performance Considerations & Optimization Opportunities

Gravity is designed to scale, but there are important performance considerations in its current implementation.

## 🔒 Lock Contention

All entries are pushed into a shared:

```rust
Arc<Mutex<BinaryHeap<Entry>>>
```

This means:

* Every file and directory requires acquiring a mutex lock.
* Large directory trees may cause significant lock contention.
* Concurrency benefits can be reduced due to synchronization overhead.

### Potential Improvements

* Use a lock-free or sharded structure
* Maintain thread-local heaps and merge at the end
* Use a bounded heap that only keeps top N entries
* Replace `Mutex` with `RwLock` if appropriate

---

## 📈 Unbounded Memory Growth

Currently:

* Every file and directory is pushed into the heap
* Memory usage grows with filesystem size

For very large drives, this can consume substantial RAM.

### Potential Improvements

* Maintain a fixed-size min-heap of top N elements
* Stream results instead of storing all entries
* Add size threshold filtering

---

## 🧵 Task Spawning Overhead

Each subdirectory spawns a new async task using `JoinSet`.

In very deep or wide directory trees:

* Task creation overhead increases
* Scheduler pressure increases
* Stack growth and future boxing adds cost

### Potential Improvements

* Use a bounded task pool
* Switch to iterative traversal with a work queue
* Add concurrency limits (e.g., semaphore)

---

## 📂 I/O Bound Behavior

Filesystem scanning is heavily I/O-bound:

* Performance depends on disk speed (SSD vs HDD)
* Metadata calls (`symlink_metadata`) are expensive
* Network filesystems may dramatically reduce performance

### Potential Improvements

* Batch metadata operations where possible
* Reduce syscalls
* Add optional depth limiting
* Add ignore rules to skip heavy directories

---

## 🧮 Sorting Strategy

`BinaryHeap` ordering currently prioritizes:

1. `size_excluding_max`
2. `total_size`
3. `path`

This ordering is more computationally expensive than a simple size comparison.

### Potential Improvements

* Benchmark alternative ordering strategies
* Make ranking strategy configurable
* Precompute and cache comparison keys

---

## 🖥️ Scaling Concerns

On very large filesystems (millions of files):

* Heap operations become costly (`O(log n)` per push)
* Memory pressure increases
* Async overhead may outweigh benefits

Long-term improvements may include:

* Streaming aggregation
* Chunked processing
* Parallel filesystem partitioning
* Dedicated performance benchmarks

---

## 🔍 Current Limitations

* Path is hardcoded to `/home`
* No CLI arguments yet
* No filtering options
* No progress reporting
* No output formatting (raw debug print only)
* No Windows testing yet
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

### Performance

* [ ] Replace global heap with bounded top-N structure
* [ ] Reduce lock contention
* [ ] Add concurrency limits
* [ ] Add benchmarking suite
* [ ] Profile with large datasets

### UX Improvements

* [ ] Pretty terminal output (tables or colors)
* [ ] Add progress indicator
* [ ] Add interactive mode
* [ ] Export results to JSON/CSV

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
