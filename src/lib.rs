// A base in-memory cache library with bounded capacity.
// Future support will include eviction policies, TTL expiration, and thread safety.

use std::collections::HashMap;

pub struct Cache<K, V> {
    capacity: usize,
    store: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.store.get(key)
        // TODO: with LRU this will need to update usage stats
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.store.len() >= self.capacity {
            // TODO: implement eviction policy (e.g., FIFO, LRU, LFU)
        }

        self.store.insert(key, value);
    }
}