use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node<K> {
    pub key: K,
    pub prev: Option<Weak<RefCell<Node<K>>>>,
    pub next: Option<Rc<RefCell<Node<K>>>>,
}

impl<K> Node<K> {
    pub fn new(key: K) -> Self {
        Self {
            key,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<K> {
    head: Option<Rc<RefCell<Node<K>>>>,
    tail: Option<Rc<RefCell<Node<K>>>>,
}

impl<K: Clone + Eq> LinkedList<K> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, key: K) -> Rc<RefCell<Node<K>>> {
        let new_node = Rc::new(RefCell::new(Node::new(key)));
        
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
        self.tail = Some(Rc::clone(&new_node));

        Rc::clone(&new_node)
    }

    pub fn pop_front(&mut self) -> Option<K> {
        self.head.take().map(|old_head| {
            let key = old_head.borrow().key.clone();
            self.head = old_head.borrow_mut().next.take(); 
            if let Some(new_head) = &self.head {
                new_head.borrow_mut().prev = None;
            } else {
                self.tail = None;
            }

            key
        })
    }

    pub fn remove_node(&mut self, node: &Rc<RefCell<Node<K>>>) {
        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();

        // link prev -> next
        if let Some(ref prev_weak) = prev {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = next.clone();
            }
        } else {
            // node was a head
            self.head = next.clone()
        }

        // link next -> prev
        if let Some(next_rc) = &next {
            next_rc.borrow_mut().prev = prev.clone();
        } else {
            // node was tail
            self.tail = prev.and_then(|w| w.upgrade());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}