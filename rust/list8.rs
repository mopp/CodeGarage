#![feature(shared)]

use std::ptr;
use std::ptr::Shared;
use std::ops::{Deref, DerefMut};

pub struct List<T> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    length: usize,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List<T> {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn push_head(&mut self, n: Node<T>) {
        match self.head {
            None => self.head = Some(unsafe { Shared }),
            Some(head) => {
                n.next = self.head;
                head.next.
            }
        }
    }

    pub fn push_tail(&self, n: Node<T>) {
    }
}

struct Node<T> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    v: T
}

impl<T> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node {
            next: None,
            prev: None,
            v
        }
    }

    fn length(&self) -> usize {
        let mut node =
            if let Some(ref next) = self.next {
                unsafe { next.as_ref() }
            } else {
                return 1;
            };

        let mut count = 2;
        loop {
            match node.next {
                None => break,
                Some(ref next) => {
                    node = unsafe { next.as_ref() };
                    count += 1;
                }
            }
        }

        count
    }
}


impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.v
    }
}


impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.v
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    struct Frame {
        order: usize,
        is_used: bool,
    }

    #[test]
    fn test_usage() {
        let mut n1: Node<Frame> = Node::new(Frame {order: 0, is_used: false});

        assert_eq!(n1.order, 0);
        n1.order = 10;
        assert_eq!(n1.order, 10);

        assert_eq!(n1.length(), 1);
    }
}
