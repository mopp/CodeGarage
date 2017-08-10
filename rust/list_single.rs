#![feature(unique)]
#![feature(allocator_api, heap_api)]
use std::ptr::Unique;
use std::heap::{Alloc, System, Layout};

type UniqueNode<T> = Unique<Node<T>>;

struct List<T> {
    head: Option<UniqueNode<T>>,
}

struct Node<T> {
    value: T,
    next: Option<UniqueNode<T>>,
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T>
    {
        Node {
            value: v,
            next: None
        }
    }

    fn with_next(v: T, next: Option<UniqueNode<T>>) -> Node<T>
    {
        Node {
            value: v,
            next: next
        }
    }

    fn as_ref(&self) -> &T
    {
        &self.value
    }

    fn as_mut(&mut self) -> &mut T
    {
        &mut self.value
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
        }
    }

    fn push_front(&mut self, node: &mut Node<T>)
    {
        let old_head = self.head;
        node.next = old_head;
        self.head = Unique::new(node as *mut _);
    }

    fn front(&self) -> Option<&T>
    {
        match self.head {
            None             => None,
            Some(ref unique) => unsafe { Some(unique.as_ref().as_ref()) }
        }
    }

    fn front_mut(&mut self) -> Option<&mut T>
    {
        match self.head {
            None             => None,
            Some(ref mut unique) => unsafe { Some(unique.as_mut().as_mut()) }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front()
    {
        let mut list: List<usize> = List::new();

        let mut n1 = Node::new(101);
        let mut n2 = Node::new(102);
        let mut n3 = Node::new(103);

        list.push_front(&mut n1);
        list.push_front(&mut n2);
        list.push_front(&mut n3);

        assert_eq!(list.front(), Some(&103));
    }
}
