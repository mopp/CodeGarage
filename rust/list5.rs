#![crate_type = "rlib"]
#![crate_name = "list5"]
#![feature(unique)]
#![feature(ptr_internals)]
#![cfg_attr(test, feature(allocator_api))]

use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, Unique};
use std::mem;

/// LinkedList struct.
pub struct LinkedList<T> {
    // There two fields are just dummy node to implement Node::detach easily.
    head: Node<T>,
    tail: Node<T>,
}

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        let head = unsafe {
            let mut n: Node<T> = mem::uninitialized();
            n.next = None;
            n.prev = None;
            n
        };

        let tail = unsafe {
            let mut n: Node<T> = mem::uninitialized();
            n.next = None;
            n.prev = None;
            n
        };

        LinkedList {
            head: head,
            tail: tail,
        }
    }

    pub fn len(&self) -> usize {
        let mut node = if let Some(ref head) = self.head.next {
            unsafe { head.as_ref() }
        } else {
            return 0;
        };

        let mut cnt = 1;
        loop {
            match node.next {
                None => break,
                Some(ref next) => {
                    node = unsafe { next.as_ref() };
                    cnt += 1;
                }
            }
        }
        cnt
    }

    pub fn head(&self) -> Option<&T> {
        unsafe { self.head.next.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head
                .next
                .as_mut()
                .map(|node| &mut node.as_mut().element)
        }
    }

    pub fn tail(&self) -> Option<&T> {
        unsafe { self.tail.prev.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail
                .prev
                .as_mut()
                .map(|node| &mut node.as_mut().element)
        }
    }

    pub fn push_head(&mut self, new_node: Unique<Node<T>>) {
        let mut new_head = NonNull::from(new_node);

        {
            let n = unsafe { new_head.as_mut() };
            n.next = self.head.next;
            n.prev = None;
        }

        let new_head = Some(new_head);
        match self.head.next {
            None => self.tail.prev = new_head,
            Some(mut old_head) => unsafe { old_head.as_mut().prev = new_head },
        }

        self.head.next = new_head;
    }

    pub fn push_tail(&mut self, new_node: Unique<Node<T>>) {
        let mut new_node = NonNull::from(new_node);

        {
            let n = unsafe { new_node.as_mut() };
            n.next = None;
            n.prev = self.tail.prev;
        }

        let new_node = Some(new_node);
        match self.tail.prev {
            None => self.head.next = new_node,
            Some(mut tail) => unsafe { tail.as_mut().next = new_node },
        }

        self.tail.prev = new_node;
    }

    pub fn pop_head(&mut self) -> Option<Unique<Node<T>>> {
        match self.head.next {
            None => None,
            Some(head) => {
                self.head.next = unsafe { head.as_ref().next };

                match self.head.next {
                    None => self.tail.prev = None,
                    Some(mut new_head) => unsafe { new_head.as_mut().prev = None },
                }

                unsafe { Some(Unique::new_unchecked(head.as_ptr())) }
            }
        }
    }

    pub fn pop_tail(&mut self) -> Option<Unique<Node<T>>> {
        match self.tail.prev {
            None => None,
            Some(tail) => {
                self.tail.prev = unsafe { tail.as_ref().prev };

                match self.tail.prev {
                    None => self.head.next = None,
                    Some(mut new_tail) => unsafe { new_tail.as_mut().next = None },
                }

                unsafe { Some(Unique::new_unchecked(tail.as_ptr())) }
            }
        }
    }
}

impl<T> Node<T> {
    pub fn detach(&mut self) {
        if let Some(mut next) = self.next {
            let next = unsafe { next.as_mut() };
            next.prev = self.prev;
        }

        if let Some(mut prev) = self.prev {
            let prev = unsafe { prev.as_mut() };
            prev.next = self.next;
        }

        self.next = None;
        self.prev = None;
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.element
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.element
    }
}

trait Getter<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

impl<T> Getter<T> for Unique<Node<T>> {
    fn get(&self) -> &T {
        unsafe { self.as_ref().deref() }
    }

    fn get_mut(&mut self) -> &mut T {
        unsafe { self.as_mut().deref_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, Layout, System};
    use std::slice;

    #[derive(PartialEq, Eq, Debug, Clone)]
    struct Frame {
        order: u8,
        is_alloc: bool,
    }

    fn allocate_node_objs<'a, T>(count: usize) -> &'a mut [T] {
        let type_size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let layout = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();
        unsafe { slice::from_raw_parts_mut(ptr as *mut T, count) }
    }

    #[test]
    fn test_new() {
        let mut list = LinkedList::<Frame>::new();

        assert_eq!(list.len(), 0);
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert_eq!(list.head_mut(), None);
        assert_eq!(list.tail_mut(), None);
        assert_eq!(list.pop_head().is_none(), true);
        assert_eq!(list.pop_tail().is_none(), true);
    }

    #[test]
    fn test_push_head() {
        const SIZE: usize = 32;
        let objs = allocate_node_objs::<Node<Frame>>(SIZE);

        let mut cnt = 0;
        for f in objs.as_mut() {
            f.order = cnt;
            f.is_alloc = false;
            cnt += 1;
        }

        let mut list = LinkedList::new();

        {
            list.push_head(unsafe { Unique::new_unchecked(&mut objs[0]) });

            let f = &objs[0];
            assert_eq!(list.len(), 1);
            assert_eq!(list.tail().unwrap(), f.deref());
            assert_eq!(list.head().unwrap(), f.deref());

            assert_eq!(list.pop_tail().unwrap().get(), f.deref());

            assert_eq!(list.len(), 0);
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert_eq!(list.head_mut(), None);
            assert_eq!(list.tail_mut(), None);
            assert_eq!(list.pop_head().is_none(), true);
            assert_eq!(list.pop_tail().is_none(), true);
        }

        {
            list.push_head(unsafe { Unique::new_unchecked(&mut objs[0]) });

            let f = &objs[0];
            assert_eq!(list.len(), 1);
            assert_eq!(list.tail().unwrap(), f.deref());
            assert_eq!(list.head().unwrap(), f.deref());

            assert_eq!(list.pop_tail().unwrap().get(), f.deref());

            assert_eq!(list.len(), 0);
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert_eq!(list.head_mut(), None);
            assert_eq!(list.tail_mut(), None);
            assert_eq!(list.pop_head().is_none(), true);
            assert_eq!(list.pop_tail().is_none(), true);
        }

        {
            list.push_head(unsafe { Unique::new_unchecked(&mut objs[0]) });
            list.push_head(unsafe { Unique::new_unchecked(&mut objs[1]) });
            list.push_head(unsafe { Unique::new_unchecked(&mut objs[2]) });
            let f0 = &objs[0];
            let f1 = &objs[1];
            let f2 = &objs[2];

            assert_eq!(list.len(), 3);
            assert_eq!(list.tail().unwrap(), f0.deref());
            assert_eq!(list.head().unwrap(), f2.deref());

            assert_eq!(list.pop_head().unwrap().get(), f2.deref());
            assert_eq!(list.pop_tail().unwrap().get(), f0.deref());

            assert_eq!(list.len(), 1);
            assert_eq!(list.tail().unwrap(), f1.deref());
            assert_eq!(list.head().unwrap(), f1.deref());
        }
    }

    #[test]
    fn test_push_tail() {
        const SIZE: usize = 32;
        let objs = allocate_node_objs::<Node<Frame>>(SIZE);

        let mut cnt = 0;
        for f in objs.as_mut() {
            f.order = cnt;
            f.is_alloc = false;
            cnt += 1;
        }

        let mut list = LinkedList::new();

        {
            list.push_tail(unsafe { Unique::new_unchecked(&mut objs[0]) });

            let f = &objs[0];
            assert_eq!(list.len(), 1);
            assert_eq!(list.tail().unwrap(), f.deref());
            assert_eq!(list.head().unwrap(), f.deref());

            assert_eq!(list.pop_tail().unwrap().get(), f.deref());

            assert_eq!(list.len(), 0);
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert_eq!(list.head_mut(), None);
            assert_eq!(list.tail_mut(), None);
            assert_eq!(list.pop_head().is_none(), true);
            assert_eq!(list.pop_tail().is_none(), true);
        }

        {
            list.push_tail(unsafe { Unique::new_unchecked(&mut objs[0]) });

            let f = &objs[0];
            assert_eq!(list.len(), 1);
            assert_eq!(list.tail().unwrap(), f.deref());
            assert_eq!(list.head().unwrap(), f.deref());

            assert_eq!(list.pop_tail().unwrap().get(), f.deref());

            assert_eq!(list.len(), 0);
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert_eq!(list.head_mut(), None);
            assert_eq!(list.tail_mut(), None);
            assert_eq!(list.pop_head().is_none(), true);
            assert_eq!(list.pop_tail().is_none(), true);
        }

        {
            list.push_tail(unsafe { Unique::new_unchecked(&mut objs[0]) });
            list.push_tail(unsafe { Unique::new_unchecked(&mut objs[1]) });
            list.push_tail(unsafe { Unique::new_unchecked(&mut objs[2]) });
            let f0 = &objs[0];
            let f1 = &objs[1];
            let f2 = &objs[2];

            assert_eq!(list.len(), 3);
            assert_eq!(list.head().unwrap(), f0.deref());
            assert_eq!(list.tail().unwrap(), f2.deref());

            assert_eq!(list.pop_tail().unwrap().get(), f2.deref());
            assert_eq!(list.pop_head().unwrap().get(), f0.deref());

            assert_eq!(list.len(), 1);
            assert_eq!(list.tail().unwrap(), f1.deref());
            assert_eq!(list.head().unwrap(), f1.deref());
        }
    }

    #[test]
    fn test_pop_head() {
        let objs = allocate_node_objs::<Node<usize>>(128);
        let size = objs.len();

        let mut list = LinkedList::new();
        for (index, node) in objs.iter_mut().enumerate() {
            node.element = index;

            let unique = unsafe { Unique::new_unchecked(node) };
            list.push_head(unique);
        }

        assert_eq!(list.len(), objs.len());
        assert_eq!(list.tail(), Some(&0usize));
        assert_eq!(list.head(), Some(&(size - 1)));

        for i in (0..objs.len()).rev() {
            assert_eq!(&i, list.pop_head().unwrap().get());
        }
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_accessors() {
        let objs = allocate_node_objs::<Node<usize>>(128);

        let mut list = LinkedList::new();
        for (i, o) in objs.iter_mut().enumerate() {
            o.element = i;

            list.push_head(unsafe { Unique::new_unchecked(o) });
        }

        assert_eq!(list.len(), objs.len());
        assert_eq!(list.tail_mut(), Some(&mut 0));
        assert_eq!(list.pop_tail().is_some(), true);

        *list.head_mut().unwrap() = 10;
        assert_eq!(list.head(), Some(&10));
        assert_eq!(list.pop_head().unwrap().get(), &10);
        assert_eq!(list.len(), objs.len() - 2);

        objs[1].detach();
        assert_eq!(list.len(), objs.len() - 3);
        assert_eq!(
            list.head.next.unwrap().as_ptr(),
            &mut objs[128 - 2] as *mut _
            );
        assert_eq!(list.head.prev.is_none(), true);
        assert_eq!(list.tail.next.is_none(), true);
        assert_eq!(list.tail.prev.unwrap().as_ptr(), &mut objs[1] as *mut _);

        *objs[0] = 10;
        assert_eq!(*objs[0], 10);
    }

    #[test]
    fn test_usage() {
        const SIZE: usize = 128;

        let objs = allocate_node_objs::<Node<Frame>>(SIZE);

        let mut list1 = LinkedList::new();
        for f in objs {
            f.order = 0;
            f.is_alloc = false;

            list1.push_head(unsafe { Unique::new_unchecked(f) });
        }
        assert_eq!(list1.len(), SIZE);

        match list1.head() {
            Some(frame) => assert_eq!(frame.order, 0),
            None => panic!("error"),
        }

        // Move the all element into list2 from list1.
        let mut list2 = LinkedList::new();
        loop {
            match list1.pop_head() {
                Some(n) => list2.push_tail(n),
                None => break,
            }
        }
        assert_eq!(list1.len(), 0);
        assert_eq!(list2.len(), SIZE);
    }
}
