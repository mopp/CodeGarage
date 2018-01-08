#![feature(shared)]

use std::ptr;
use std::ptr::Shared;
use std::ops::{Deref, DerefMut};

pub struct List<T> {
    node: Option<Node<T>>,
    length: usize,
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
        if self.next.is_none() && self.prev.is_none() {
            return 1;
        }

        debug_assert!(self.next.is_some() && self.prev.is_some());

        let tail = self.prev.unwrap();
        let mut current = self.next.unwrap();

        let mut count = 1;
        while ptr::eq(current.as_ptr(), tail.as_ptr()) == false {
            count += 1;
            current = unsafe { current.as_ref().next.unwrap() };
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
