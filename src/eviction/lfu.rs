// use std::collections::HashMap;
// use std::hash::Hash;
// use std::cell::RefCell;
// use std::rc::Rc;
// use crate::linked_list::{LinkedList, Node};

// use super::EvictionPolicy;

// pub struct LfuPolicy<K> {
//     freq_map: HashMap<usize, LinkedList<K>>,
//     counts: HashMap<K, (usize, Rc<RefCell<Node<K>>>)>,
//     min_freq: usize,
// }

// impl<K: Eq + Hash + Clone> LfuPolicy<K> {
//     pub fn new() -> Self {
//         Self {
//             freq_map: HashMap::new(),
//             counts: HashMap::new(),
//             min_freq: 0,
//         }
//     }
// }

// impl<K: Eq + Hash + Clone> EvictionPolicy<K> for LfuPolicy<K> {
//     fn record_access(&mut self, key: &K) {
//         // TODO:
//         // - Get count and node from counts
//         // - Remove node from old freq list
//         // - If old freq list empty and min_freq == old freq, update min_freq
//         // - Insert into new freq list with count + 1
//     }

//     fn record_insertion(&mut self, key: &K) {
//         // TODO:
//         // - Insert key into freq_map[1]
//         // - Update counts map
//         // - Set min_freq = 1
//     }

//     fn evict(&mut self) -> Option<K> {
//         // TODO:
//         // - Find key from freq_map[min_freq]
//         // - Remove from freq_map and counts
//         // - If freq_map[min_freq] empty after, update min_freq
//     }
// }