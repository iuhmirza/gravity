use std::collections::BinaryHeap;

use crate::entry::Entry;

pub struct Collector {
    heap: BinaryHeap<Entry>,
}

impl Collector {
    fn new() -> Collector {
        Collector {heap: BinaryHeap::new()}
    }
    
    fn insert(&mut self, entry: Entry) {
        self.heap.push(entry)
    }
    
    fn collect(self) -> Vec<Entry> {
        self.heap.into_sorted_vec()
    }
}

