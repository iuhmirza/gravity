use std::{cmp::Reverse, collections::BinaryHeap};

use crate::entry::Entry;

pub struct VecCollector {
    vec: Vec<Entry>,
    max_size: usize,
}

pub struct HeapCollector {
    heap: BinaryHeap<Reverse<Entry>>,
    max_size: usize,
}

pub trait Collector {
    fn new() -> impl Collector;
    fn insert(&mut self, entry: Entry); 
    fn collect(&self) -> Vec<Entry>;
}

impl Collector for HeapCollector{
    fn new() -> impl Collector {
        HeapCollector {heap: BinaryHeap::new(), max_size: 8}
    }
    
    fn insert(&mut self, entry: Entry) {
        self.heap.push(Reverse(entry));
        if self.heap.len() > self.max_size {
            self.heap.pop();
        }
    }
    
    fn collect(&self) -> Vec<Entry> {
        self.heap
            .clone()
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(x)| x)
            .collect()
    }
}

impl Collector for VecCollector {
    fn new() -> impl Collector {
        VecCollector {max_size: 8, vec: Vec::new()}
    }
    
    fn insert(&mut self, entry: Entry) {
        self.vec.push(entry);
    }
    
    fn collect(&self) -> Vec<Entry> {
        let mut vec = self.vec.clone();
        vec.sort();
        vec.reverse();
        vec.truncate(8);
        vec
    }
    
}