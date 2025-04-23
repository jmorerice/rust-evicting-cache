// A base in-memory cache library with bounded capacity.
// Future support will include eviction policies, TTL expiration, and thread safety.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type Link<K> = Option<Rc<RefCell<Node<K>>>>;

pub struct LruList<K> {
    head: Link<K>,                      // Most recently used
    tail: Link<K>,                      // Least recently used
}

struct Node<K> {
    key: K,
    prev: Link<K>,
    next: Link<K>,
}

// Trait for pluggable eviction policies
pub trait EvictionPolicy<K> {
    // Called on every key access (for LRU/LFU updates)
    fn record_access(&mut self, key: &K);
    fn record_insertion(&mut self, key: &K);
    fn record_removal(&mut self, key: &K);
    fn evict_candidate(&mut self) -> Option<K>;
}

pub struct Cache<K, V, P> 
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    P: EvictionPolicy<K>,
{
    capacity: usize,
    store: HashMap<K, V>,
    policy: P
}

impl <K: Clone> LruList<K> {
    pub fn new() -> Self {
        LruList {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, key: K) -> Rc<RefCell<Node<K>>> {
        let new_node = Rc::new(RefCell::new(Node {
            key,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
            None => {
                // If empty, both head and tail point to new node
                self.tail = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
        }

        new_node
    }
}

impl<K, V, P> Cache<K, V, P> 
where 
    K: std::cmp::Eq + std::hash::Hash + Clone,
    P: EvictionPolicy<K>,
{
    pub fn new(capacity: usize, policy: P) -> Self {
        Cache {
            capacity,
            store: HashMap::new(),
            policy,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.policy.record_access(key); //Track recency
        self.store.get(key)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.store.len() >= self.capacity && !self.store.contains_key(&key) {
            if let Some(evict_key) = self.policy.evict_candidate() {
                self.store.remove(&evict_key);
                self.policy.record_removal(&evict_key);
            }
        }

        self.store.insert(key.clone(), value);
        self.policy.record_insertion(&key)
    }
}