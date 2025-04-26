use std::collections::VecDeque;
use super::EvictionPolicy;

pub struct FifoPolicy<K> {
    queue: VecDeque<K>,
}

impl<K: Clone + Eq> FifoPolicy<K> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    } 
}

impl <K: Clone + Eq> EvictionPolicy<K> for FifoPolicy<K> {
    fn record_insertion(&mut self, key: &K) {
        self.queue.push_back(key.clone());
    }
    // Do nothing, does not apply to FIFO
    fn record_access(&mut self, key: &K) {}

    fn evict(&mut self) -> Option<K> {
        self.queue.pop_front()
    }
}