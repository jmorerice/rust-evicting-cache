use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

use super::EvictionPolicy;

struct Node<K> {
    key: K,
    prev: Option<Weak<RefCell<Node<K>>>>,
    next: Option<Rc<RefCell<Node<K>>>>,
}

impl<K> Node<K> {
    fn new(key: K) -> Self {
        Node {
            key,
            prev: None,
            next: None,
        }
    }
}

struct LruList<K> {
    head: Option<Rc<RefCell<Node<K>>>>,
    tail: Option<Rc<RefCell<Node<K>>>>,
}

impl<K: Clone + Eq> LruList<K> {
    fn new() -> Self {
        LruList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, key: K) -> Rc<RefCell<Node<K>>> {
        let new_node = Rc::new(RefCell::new(Node::new(key)));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
            }
        }

        self.head = Some(Rc::clone(&new_node));
        Rc::clone(&new_node)
    }

    fn move_to_front(&mut self, node: &Rc<RefCell<Node<K>>>) {
        if Some(Rc::as_ptr(node)) == self.head.as_ref().map(|h| Rc::as_ptr(h)) {
            return;
        }

        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();

        if let Some(prev_weak) = &prev {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = next.clone();
            }
        }

        if let Some(next_rc) = &next {
            next_rc.borrow_mut().prev = prev;
        } else {
            self.tail = prev.as_ref().and_then(|w| w.upgrade());
        }

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().prev = Some(Rc::downgrade(node));
            node.borrow_mut().next = Some(old_head);
        }

        self.head = Some(Rc::clone(node));
    }

    fn remove_lru(&mut self) -> Option<K> {
        self.tail.take().map(|old_tail| {
            let key = old_tail.borrow().key.clone();

            if let Some(prev_weak) = &old_tail.borrow().prev {
                if let Some(prev_rc) = prev_weak.upgrade() {
                    prev_rc.borrow_mut().next = None;
                    self.tail = Some(prev_rc);
                }
            } else {
                self.head = None;
            }

            key
        })
    }
}

pub struct LruPolicy<K> {
    list: LruList<K>,
    map: HashMap<K, Rc<RefCell<Node<K>>>>,
}

impl<K: Eq + Hash + Clone> LruPolicy<K> {
    pub fn new() -> Self {
        Self {
            list: LruList::new(),
            map: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash + Clone> EvictionPolicy<K> for LruPolicy<K> {
    fn record_access(&mut self, key: &K) {
        if let Some(node) = self.map.get(key) {
            self.list.move_to_front(node);
        }
    }

    fn record_insertion(&mut self, key: &K) {
        let node = self.list.push_front(key.clone());
        self.map.insert(key.clone(), node);
    }

    fn evict(&mut self) -> Option<K> {
        let key = self.list.remove_lru()?;
        self.map.remove(&key);
        Some(key)
    }
}