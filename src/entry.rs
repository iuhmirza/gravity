use std::path::PathBuf;

#[derive(Debug)]
pub struct Entry {
    path: PathBuf,
    kind: EntryKind,
    total_size: u64,
    size_excluding_max: u64,
}

impl Entry {
    pub fn new(path: PathBuf, kind: EntryKind, total_size: u64, net_size: u64) -> Entry {
        Entry {path, kind, total_size, size_excluding_max: net_size}
    }
    pub fn size(&self) -> u64 {
        self.total_size
    }
}

#[derive(Debug)]
pub enum EntryKind {
    File,
    Directory,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.size_excluding_max == other.size_excluding_max
            && self.total_size == other.total_size
            && self.path == other.path
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> std::cmp::Ordering {
        self.size_excluding_max
            .cmp(&other.size_excluding_max)
            .then(self.total_size.cmp(&other.total_size))
            .then(self.path.cmp(&other.path))
    }
}
