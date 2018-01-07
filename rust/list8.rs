#![feature(shared)]

use std::ptr::Shared;
use std::ops::{Deref, DerefMut};


struct Node<T> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    v: T
}

impl<T> Node<T> {
    fn init_link(&mut self) {
        let s = unsafe {Shared::new_unchecked(self as _)};
        self.next = Some(s);
        self.prev = Some(s);
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


impl<T> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node {
            next: None,
            prev: None,
            v
        }
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
    }
}
