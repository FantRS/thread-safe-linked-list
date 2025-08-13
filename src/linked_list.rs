use std::{
    fmt::Debug,
    sync::{Arc, Mutex, PoisonError},
};

type Error = LinkedListError;

struct Node<T> {
    value: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }

    fn safe_new(value: T) -> Arc<Mutex<Node<T>>> {
        Arc::new(Mutex::new(Node::new(value)))
    }
}

pub struct LinkedList<T> {
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
    len: u32,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn push_back(&mut self, value: T) -> Result<(), Error> {
        let node = Node::safe_new(value);

        if self.is_empty() {
            self.head = Some(Arc::clone(&node));
            self.tail = Some(Arc::clone(&node));
            self.len += 1;

            return Ok(());
        }

        if let Some(old_tail) = self.tail.take() {
            old_tail.lock()?.next = Some(Arc::clone(&node));
            node.lock()?.prev = Some(Arc::clone(&old_tail));

            self.tail = Some(node);
            self.len += 1;
        }

        Ok(())
    }

    pub fn push_front(&mut self, value: T) -> Result<(), Error> {
        let node = Node::safe_new(value);

        if self.is_empty() {
            self.head = Some(Arc::clone(&node));
            self.tail = Some(Arc::clone(&node));
            self.len += 1;

            return Ok(());
        }

        if let Some(old_head) = self.head.take() {
            old_head.lock()?.prev = Some(Arc::clone(&node));
            node.lock()?.next = Some(Arc::clone(&old_head));

            self.head = Some(node);
            self.len += 1;
        }

        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<()> {
        if self.is_empty() {
            return None;
        }

        if let Some(old_tail) = self.tail.take() {
            if let Some(prev_of_tail) = old_tail.lock().ok()?.prev.take() {
                prev_of_tail.lock().ok()?.next = None;

                self.tail = Some(Arc::clone(&prev_of_tail));
                self.len -= 1;

                return Some(());
            }

            self.head = None;
            self.tail = None;
            self.len = 0;
        };

        Some(())
    }

    pub fn pop_front(&mut self) -> Option<()> {
        if self.is_empty() {
            return None;
        };

        if let Some(old_head) = self.head.take() {
            if let Some(next_of_head) = old_head.lock().ok()?.next.take() {
                next_of_head.lock().ok()?.prev = None;

                self.head = Some(Arc::clone(&next_of_head));
                self.len -= 1;

                return Some(());
            }

            self.head = None;
            self.tail = None;
            self.len = 0;
        };

        Some(())
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}

impl<T: Clone> LinkedList<T> {
    pub fn get_head(&self) -> Option<T> {
        if let Some(head) = &self.head {
            return Some(head.lock().ok()?.value.clone());
        }

        None
    }

    pub fn get_tail(&self) -> Option<T> {
        if let Some(tail) = &self.tail {
            return Some(tail.lock().ok()?.value.clone());
        }

        None
    }
}

impl<T: Debug> LinkedList<T> {
    pub fn display_list(&self) {
        if self.is_empty() {
            println!("empty list");
            return;
        }

        let mut current_opt = self.head.clone();

        while let Some(current) = current_opt {
            let node = current.lock().unwrap();
            println!("{:?}", node.value);
            current_opt = node.next.clone();
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LinkedListError {
    #[error("MutexGuard unwrapping error")]
    MutexGuardError,
}

impl<T> From<PoisonError<T>> for LinkedListError {
    fn from(_value: PoisonError<T>) -> Self {
        Self::MutexGuardError
    }
}
