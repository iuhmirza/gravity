# TODO: Gravity Development Roadmap 🚀

This document tracks identified bottlenecks, architectural questions, and planned features to improve the "Gravity" engine.

## ⚡ Performance & Optimization

* [ ] **Solve Mutex Contention:** In `scanner.rs`, the async scan currently locks a `Mutex` for *every single file found*. This will likely cause a massive bottleneck on fast NVMe drives.
* *Question:* Should we use a `mpsc` channel to send entries to a dedicated collector task instead?


* [ ] **Task Granularity:** Currently, every directory spawns a new `tokio` task. For deep, fragmented file systems (like `node_modules`), this creates thousands of tasks.
* *Improvement:* Implement a depth-based threshold where the scanner switches from async spawning to a standard recursive loop to reduce runtime overhead.


* [ ] **Memory Management:** The `VecCollector` stores *all* entries before truncating. On a drive with millions of files, this will spike RAM usage.
* *Improvement:* Refine `HeapCollector` to be the default to keep memory footprint constant (O(max_size)).


* [ ] **Metadata Optimization:** Explore `low-level` platform-specific syscalls (like `getdents64` on Linux) to fetch metadata during the directory crawl rather than making separate `metadata()` calls for every file.

## 🎨 User Experience (UX)

* [ ] **CLI Argument Parsing:** Replace the hardcoded `/home` path with `clap` for proper argument handling.
* *Feature:* Allow users to specify `--path`, `--limit` (top N results), and `--engine` (sync vs async).


* [ ] **Human-Readable Output:** - *Improvement:* Convert raw byte counts into auto-scaling units (B, KB, MB, GB, TB).
* *Improvement:* Add color-coding based on the "Gravity" (disproportionate size) of a folder.


* [ ] **Progress Monitoring:** Use the `indicatif` crate to show a real-time progress bar or a "files scanned" counter.
* [ ] **Ignore Patterns:** Add support for `.gravityignore` or standard `.gitignore` patterns to skip large directories like `.git` or `target`.

## 🧠 Architectural Questions

* [ ] **Defining "Gravity":** Is `total_size - max_element_size` the most effective way to find bloat?
* *Experiment:* Would a "Density" metric (Total Size / File Count) be more useful for finding cache graveyards?


* [ ] **Cross-Platform Handling:** How should we handle "Hidden" files on Windows vs. Unix, or filesystems with different block sizes?
* [ ] **Error Resilience:** Currently, one `io::Error` (like Permission Denied) can halt the scan.
* *Improvement:* Implement a logging system that skips restricted folders while continuing the scan.



## 🛠️ Refactoring

* [ ] Move `scanner` logic into a trait-based system to allow for different traversal strategies (BFS vs DFS).
* [ ] Add comprehensive benchmarks using `criterion` to compare `sync` vs `async` across different hardware (HDD vs SSD).
