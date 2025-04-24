use std::collections::HashMap;
use crate::eviction::EvictionPolicy;

pub struct Cache<K, V, P>
where
    K: Eq + std::hash::Hash + Clone,
    P: EvictionPolicy<K>
{
    capacity: usize,
    store: HashMap<K, V>,
    policy: P,
}

impl<K, V, P> Cache<K, V, P> 
where 
    K: Eq + std::hash::Hash + Clone,
    P: EvictionPolicy<K>,
{
    pub fn new(capacity: usize, policy: P) -> Self {
        Self {
            capacity,
            store: HashMap::new(),
            policy,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.policy.record_access(key);
        self.store.get(key)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.store.len() >= self.capacity && !self.store.contains_key(&key) {
            if let Some(evicted_key) = self.policy.evict() {
                self.store.remove(&evicted_key);
            }
        }
        
        self.policy.record_insertion(&key);
        self.store.insert(key, value);
    }
}