#![feature(shared)]
#![feature(unique)]
#![feature(allocator_api)]

use std::ptr::Shared;
use std::ptr::Unique;
use std::mem;
use std::ptr;


struct LinkedList<T> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    length: usize,
}


struct Node<T> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    element: T,
}


impl<T> LinkedList<T> {
    fn new() -> LinkedList<T>
    {
        LinkedList {
            head: None,
            tail: None,
            length: 0
        }
    }

    fn front(&self) -> Option<&T>
    {
        unsafe {
            self.head.as_ref().map(|node| &node.as_ref().element)
        }
    }

    fn front_mut(&mut self) -> Option<&mut T>
    {
        unsafe {
            self.head.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    fn back(&self) -> Option<&T>
    {
        unsafe {
            self.tail.as_ref().map(|node| &node.as_ref().element)
        }
    }

    fn back_mut(&mut self) -> Option<&mut T>
    {
        unsafe {
            self.tail.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    fn push_front(&mut self, new_node: *mut Node<T>)
    {
        let mut new_shared_node = unsafe { Shared::new_unchecked(new_node) };

        {
            let n = unsafe { new_shared_node.as_mut() };
            n.next = self.head;
            n.prev = None;
        }

        let new_shared_node = Some(new_shared_node);
        match self.head {
            None           => self.tail = new_shared_node,
            Some(mut head) => unsafe { head.as_mut().prev = new_shared_node },
        }

        self.head = new_shared_node;
        self.length += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, System, Layout};
    use std::slice;

    fn allocate_unique_objs<'a, T>(count: usize) -> &'a mut [T]
    {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr       = unsafe { System.alloc(layout) }.unwrap();
        unsafe { slice::from_raw_parts_mut(ptr as *mut T, count) }
    }

    #[test]
    fn test_push_front()
    {
        let objs = allocate_unique_objs::<Node<usize>>(1024);

        let mut list = LinkedList::new();
        list.push_front(&mut objs[0] as *mut _);
        // list.push_front(&mut objs[0] as *mut _);
    }
}
