use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::linked_list::{LinkedList, Node};

use super::EvictionPolicy;

pub struct LruPolicy<K> {
    list: LinkedList<K>,
    map: HashMap<K, Rc<RefCell<Node<K>>>>,
}

impl<K: Eq + Hash + Clone> LruPolicy<K> {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
            map: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash + Clone> EvictionPolicy<K> for LruPolicy<K> {
    fn record_access(&mut self, key: &K) {
        if let Some(node) = self.map.get(key) {
            self.list.remove_node(node);
            let new_node = self.list.push_back(key.clone());
            self.map.insert(key.clone(), new_node);
        }
    }

    fn record_insertion(&mut self, key: &K) {
        let node = self.list.push_back(key.clone());
        self.map.insert(key.clone(), node);
    }

    fn evict(&mut self) -> Option<K> {
        if let Some(key) = self.list.pop_front() {
            self.map.remove(&key);
            Some(key)
        } else {
            None
        }
    }
}