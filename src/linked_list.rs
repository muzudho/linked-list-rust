use crate::{Iter, LinkedList, Node};
use std::fmt;
use std::rc::Rc;

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn append(&mut self, v: T) {
        let node = Node::new(v);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                // first element
                debug_assert_eq!(self.len(), 0);
                self.head = Some(Rc::clone(&node));
            }
        }

        self.tail = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail) => {
                if let Some(prev) = tail.borrow_mut().prev.take() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                } else {
                    // we take last element
                    debug_assert_eq!(self.len(), 1);
                    self.head = None;
                }
                self.length -= 1;
                let v = Rc::try_unwrap(tail) // Rc<RefCell<Node<T>> -> RefCell<Node<T>>
                    .ok() // Result<RefCell<Node<T>>, Rc<RefCell<Node<T>>>> -> Option<RefCell<Node<T>>>
                    .expect("Failed to Rc::try_unwrap tail node") // RefCell<Node<T>>
                    .into_inner() // RefCell<Node<T>> -> Node<T>
                    .value;
                Some(v)
            }
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: if self.len() == 0 {
                None
            } else {
                Some(Rc::clone(&self.head.as_ref().unwrap()))
            },
        }
    }
}

impl<T: fmt::Display + Clone> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let iter = self.iter();
        write!(f, "{{ head")?;
        for v in iter {
            write!(f, " -> {}", v)?;
        }
        write!(f, " }}")
    }
}
