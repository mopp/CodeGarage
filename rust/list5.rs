#![crate_type = "rlib"]
#![crate_name = "list5"]
#![feature(unique)]
#![feature(ptr_internals)]
#![cfg_attr(test, feature(allocator_api))]

use std::ops::{Deref, DerefMut};
use std::ptr;
use std::ptr::{NonNull, Unique};
use std::mem;

type Link<T> = NonNull<Node<T>>;

/// LinkedList struct.
/// The right arrows refer `next`.
/// head -> Node1 -> Node2 -> tail
/// head <- Node1 <- Node2 <- tail
/// the left arrows refer `prev`.
pub struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    length: usize,
}

pub struct Node<T> {
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
    element: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn with_nodes(nodes: *mut Node<T>, count: usize) -> Option<LinkedList<T>> {
        let mut list = LinkedList::new();

        for i in 0..count {
            let n = unsafe { nodes.offset(i as isize) };

            if let Some(node) = Unique::new(n) {
                list.push_tail(node);
            } else {
                return None;
            }
        }

        return Some(list);
    }

    pub fn count_nodes(&self) -> usize {
        let mut node = match self.head {
            None => return 0,
            Some(ref head) => unsafe {head.as_ref()}
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

    pub fn len(&self) -> usize {
        debug_assert!(self.length == self.count_nodes());

        self.length
    }

    pub fn head(&self) -> Option<&T> {
        unsafe {
            self.head
                .as_ref()
                .map(|node| &node.as_ref().element)
        }
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head
                .as_mut()
                .map(|node| &mut node.as_mut().element)
        }
    }

    pub fn tail(&self) -> Option<&T> {
        unsafe {
            self.tail
                .as_ref()
                .map(|node| &node.as_ref().element)
        }
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail
                .as_mut()
                .map(|node| &mut node.as_mut().element)
        }
    }

    pub fn push_head(&mut self, new_node: Unique<Node<T>>) {
        let mut new_node = NonNull::from(new_node);

        unsafe {
            let n = new_node.as_mut();
            n.prev = None;
            n.next = None;
        };

        if let Some(mut old_head) = self.head {
            unsafe {
                new_node.as_mut().next = Some(old_head);
                old_head.as_mut().prev = Some(new_node);
            }
        } else {
            // The list has no node.
            self.tail = Some(new_node);
        }

        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn push_tail(&mut self, new_node: Unique<Node<T>>) {
        let mut new_node = NonNull::from(new_node);

        unsafe {
            let n = new_node.as_mut();
            n.prev = None;
            n.next = None;
        };

        if let Some(mut old_tail) = self.tail {
            unsafe {
                new_node.as_mut().prev = Some(old_tail);
                old_tail.as_mut().next = Some(new_node);
            }
        } else {
            // The list has no node.
            self.head = Some(new_node);
        }

        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn pop_head(&mut self) -> Option<Unique<Node<T>>> {
        match self.head {
            None => None,
            Some(head) => {
                self.head= unsafe { head.as_ref().next };

                match self.head {
                    None =>
                        self.tail = None,
                        Some(mut new_head) =>
                            unsafe { new_head.as_mut().prev = None },
                }

                self.length -= 1;
                unsafe { Some(Unique::new_unchecked(head.as_ptr())) }
            }
        }
    }

    pub fn pop_tail(&mut self) -> Option<Unique<Node<T>>> {
        match self.tail {
            None => None,
            Some(tail) => {
                self.tail = unsafe { tail.as_ref().prev };

                match self.tail {
                    None => self.head = None,
                    Some(mut new_tail) => unsafe { new_tail.as_mut().next = None },
                }

                self.length -= 1;
                unsafe { Some(Unique::new_unchecked(tail.as_ptr())) }
            }
        }
    }

    pub fn member(&self, target_node: Unique<Node<T>>) -> bool {
        let mut node = match self.head {
            None => return false,
            Some(ref head) => unsafe {head.as_ref()}
        };

        let target_node = unsafe { target_node.as_ref() };
        loop {
            if ptr::eq(node, target_node) {
                break true;
            }

            match node.next {
                None => break false,
                Some(ref next) => {
                    node = unsafe { next.as_ref() };
                }
            }
        }
    }

    pub fn detach(&mut self, mut node: Unique<Node<T>>) -> Result<(), String> {
        if self.len() == 0 {
            return Err("The list does not has any nodes.".to_string());
        }

        debug_assert!(self.head.is_some() && self.tail.is_some());
        debug_assert!(self.member(node));

        let node = unsafe {node.as_mut()};

        // Check the head equals the given node.
        match self.head {
            Some(mut head) => {
                let head = unsafe { head.as_mut() };
                if ptr::eq(head, node) {
                    self.length -= 1;
                    self.head = head.next;
                    if let Some(mut next) = head.next {
                        unsafe { next.as_mut().prev = None };
                    } else {
                        // This list has no element.
                        self.tail = None;
                    }

                    node.next = None;
                    node.prev = None;

                    return Ok(());
                }
            },
            None =>
                return Err("ERROR: no head".to_string())
        }

        // Check the tail equals the given node.
        match self.tail {
            Some(mut tail) => {
                let tail = unsafe { tail.as_mut() };
                if ptr::eq(tail, node) {
                    self.length -= 1;
                    self.tail = tail.prev;
                    if let Some(mut prev) = tail.prev {
                        unsafe { prev.as_mut().next = None };
                    } else {
                        // This list has no element.
                        self.head = None;
                    }

                    node.next = None;
                    node.prev = None;

                    return Ok(());
                }
            },
            None =>
                return Err("ERROR: no head".to_string())
        }

        if let (Some(mut next), Some(mut prev)) = (node.next,  node.prev) {
            unsafe {
                self.length -= 1;
                next.as_mut().prev = Some(prev);
                prev.as_mut().next = Some(next);
                Ok(())
            }
        } else {
            Err("ERROR: link broken.".to_string())
        }
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
    fn new() {
        let mut list = LinkedList::<Frame>::new();

        assert_eq!(list.len(), 0);
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert_eq!(list.head_mut(), None);
        assert_eq!(list.tail_mut(), None);
        assert_eq!(list.pop_head().is_none(), true);
        assert_eq!(list.pop_tail().is_none(), true);

        const SIZE: usize = 128;
        let objs = allocate_node_objs::<Node<Frame>>(SIZE);
        let mut list = LinkedList::with_nodes(&mut objs[0] as *mut _, SIZE).unwrap();
        assert_eq!(list.len(), SIZE);
        assert_eq!(list.head().is_some(), true);
        assert_eq!(list.tail().is_some(), true);
        assert_eq!(list.head_mut().is_some(), true);
        assert_eq!(list.tail_mut().is_some(), true);
        assert_eq!(list.pop_head().is_some(), true);
        assert_eq!(list.pop_tail().is_some(), true);
    }

    #[test]
    fn push_head() {
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
    fn push_tail() {
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
    fn pop_head() {
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
    fn pop_tail() {
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

        for i in 0..objs.len() {
            assert_eq!(&i, list.pop_tail().unwrap().get());
        }
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn move_nodes_into_another_list() {
        const SIZE: usize = 128;
        let objs = allocate_node_objs::<Node<Frame>>(SIZE);

        for f in objs.iter_mut() {
            f.order = 0;
            f.is_alloc = false;
        }

        let mut list1 = LinkedList::with_nodes(&mut objs[0] as *mut Node<Frame>, SIZE).unwrap();
        let mut list2 = LinkedList::new();

        assert_eq!(list1.len(), SIZE);
        assert_eq!(list2.len(), 0);

        loop {
            if let Some(n) = list1.pop_head()  {
                list2.push_tail(n)
            } else {
                break;
            }
        }

        assert_eq!(list1.len(), 0);
        assert_eq!(list2.len(), SIZE);
    }

    #[test]
    fn detach() {
        const SIZE: usize = 128;
        let objs = allocate_node_objs::<Node<Frame>>(SIZE);
        let mut list1 = LinkedList::with_nodes(&mut objs[0] as *mut Node<Frame>, SIZE).unwrap();

        assert_eq!(list1.len(), SIZE);

        let n = unsafe { Unique::new_unchecked(&mut objs[0]) };
        assert_eq!(list1.detach(n), Ok(()));
        assert_eq!(list1.len(), SIZE - 1);

        let n = unsafe { Unique::new_unchecked(&mut objs[SIZE - 1]) };
        assert_eq!(list1.detach(n), Ok(()));
        assert_eq!(list1.len(), SIZE - 2);

        let n = unsafe { Unique::new_unchecked(&mut objs[SIZE / 2]) };
        assert_eq!(list1.detach(n), Ok(()));
        assert_eq!(list1.len(), SIZE - 3);
    }
}
