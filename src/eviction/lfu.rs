use std::collections::HashMap;
use std::hash::Hash;

use super::EvictionPolicy;

pub struct LfuPolicy<K> {
    counts : HashMap<K, usize>,
}

impl<K:Clone + Eq + Hash> LfuPolicy<K> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
}

//TO DO: can be optimized (ex: min-heap (Priority Queue), Frequency Buckets)
impl <K:Clone + Eq + Hash> EvictionPolicy<K> for LfuPolicy<K> {
    fn record_access(&mut self, key: &K) {
        if let Some(count) = self.counts.get_mut(key) {
            *count += 1;
        }
    }

    fn record_insertion(&mut self, key: &K) {
        self.counts.insert(key.clone(), 1);
    }

    fn evict(&mut self) -> Option<K> {
        let mut min_count = usize::MAX;
        let mut key_to_evict = None;

        for (key, count) in self.counts.iter(){
            if *count < min_count {
                min_count = *count;
                key_to_evict = Some(key.clone());
            }
        }

        if let Some(key) = key_to_evict {
            self.counts.remove(&key);
            Some(key)
        } else {
            None
        }
    }
}