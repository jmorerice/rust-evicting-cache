use std::collections::HashMap;

// Generic Cache structure with fixed capacity
pub struct Cache<K, V> {
    capacity: usize,
    store: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Cache<K, V> {
    /// Creates a new cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            store: HashMap::new(),
        }
    }

    /// Gets a reference to a value by key, if it exists
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.store.get(key)
        // TODO: For LRU, update access order here
    }

    /// Inserts a key-value pair. Evicts if over capacity.
    pub fn put(&mut self, key: K, value: V) {
        if self.store.len() >= self.capacity {
            // TODO: implement eviction policy (e.g., FIFO, LRU, LFU)
        }

        self.store.insert(key, value);
    }
}