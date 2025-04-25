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
    // TODO: implement EvictionPolicy<K> for FifoPolicy<K>
    // - record_insertion: push_back key
    // - record_access: probably do nothing (FIFO ignores access)
    // - evict: pop_front the oldest key
}