#![cfg_attr(test, feature(allocator_api))]
#![feature(offset_to)]
#![feature(shared)]

#![crate_type = "rlib"]
#![crate_name = "list7"]

use std::ptr::Shared;
use std::ptr;

pub trait Node<T: Node<T>> {
    fn as_ptr(&mut self) -> *mut T;
    fn set_next(&mut self, Shared<T>);
    fn set_prev(&mut self, Shared<T>);
    fn next(&self) -> &T;
    fn prev(&self) -> &T;
    fn next_mut(&mut self) -> &mut T;
    fn prev_mut(&mut self) -> &mut T;

    fn as_shared(&mut self) -> Shared<T> {
        unsafe { Shared::new_unchecked(self.as_ptr()) }
    }

    fn init_link(&mut self) {
        let s = self.as_shared();
        self.set_next(s);
        self.set_prev(s);
    }

    fn length(&self) -> usize {
        let tail = self.prev();
        let mut current = self.next().prev();

        let mut count = 1;
        while ptr::eq(current as _, tail as _) == false {
            count += 1;
            current = current.next();
        }

        count
    }

    fn is_alone(&self) -> bool {
        let prev = self.prev();
        let next = self.next();
        let current = self.next().prev();

        ptr::eq(current as _, prev as _) && ptr::eq(current as _, next as _)
    }

    fn detach(&mut self) {
        let prev = self.prev().into();
        let next = self.next().into();
        println!("detach - self = {:?}", self.as_shared().as_ptr());
        println!("detach - next = {:?}", self.next_mut().as_ptr());
        println!("detach - prev = {:?}", self.prev_mut().as_ptr());
        println!("detach - len  = {:?}", self.length());
        self.next_mut().set_prev(prev);
        self.prev_mut().set_next(next);

        self.init_link();
    }

    fn find<F>(&mut self, f: F) -> Option<Shared<T>>
    where
        F: Fn(&T) -> bool,
    {
        let tail = self.prev_mut().as_shared();
        let mut current = self.next_mut().prev_mut().as_shared();

        while ptr::eq(current.as_ptr(), tail.as_ptr()) == false {
            if f(unsafe { current.as_ref() }) {
                return Some(current);
            }

            unsafe {
                current = current.as_mut().next_mut().as_shared();
            }
        }

        None
    }

    fn insert_next(&mut self, mut new_next: Shared<T>) {
        {
            let new_next = unsafe { new_next.as_mut() };
            new_next.set_next(self.next().into());
            new_next.set_prev(self.as_shared())
        }

        self.next_mut().set_prev(new_next);
        self.set_next(new_next);
    }

    fn insert_prev(&mut self, mut new_prev: Shared<T>) {
        {
            let new_prev = unsafe { new_prev.as_mut() };
            new_prev.set_next(self.as_shared());
            new_prev.set_prev(self.prev().into());
        }

        self.prev_mut().set_next(new_prev);
        self.set_prev(new_prev);
    }
}

pub struct List<T: Node<T>> {
    node: Option<Shared<T>>,
    length: usize,
}

impl<T: Node<T>> List<T> {
    pub fn new() -> List<T> {
        List {
            node: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, new_node: Shared<T>, is_next: bool) {
        if let Some(mut node) = self.node {
            unsafe {
                if is_next {
                    node.as_mut().insert_next(new_node);
                } else {
                    node.as_mut().insert_prev(new_node);
                }
            }
        } else {
            self.node = Some(new_node);
        }

        self.length += 1;
    }

    pub fn push_head(&mut self, new_node: Shared<T>) {
        self.push(new_node, true);
    }

    pub fn push_tail(&mut self, new_node: Shared<T>) {
        self.push(new_node, false);
    }

    fn pop(&mut self, is_next: bool) -> Option<Shared<T>> {
        self.node.map(|mut node| {
            if unsafe { node.as_ref().is_alone() } {
                self.node = None;
            } else {
                let node = unsafe { node.as_mut() };
                self.node = Some(
                    match is_next {
                        true => node.next_mut(),
                        false => node.prev_mut(),
                    }.as_shared(),
                );
                node.detach();
            }

            self.length -= 1;
            if self.length == 0 {
                self.node = None;
            }

            node
        })
    }

    pub fn pop_head(&mut self) -> Option<Shared<T>> {
        self.pop(true)
    }

    pub fn pop_tail(&mut self) -> Option<Shared<T>> {
        self.pop(false)
    }

    fn has_node(&self, target_node: Shared<T>) -> bool {
        match self.node {
            None => false,
            Some(node) if self.length == 1 => {
                ptr::eq(node.as_ptr(), target_node.as_ptr())
            },
            Some(mut node) => {
                let node = unsafe { node.as_mut() };
                let tail = node.prev_mut().as_shared();
                let mut current = node.next_mut().prev_mut().as_shared();

                while ptr::eq(current.as_ptr(), tail.as_ptr()) == false {
                    if ptr::eq(current.as_ptr(), target_node.as_ptr()) {
                        return true;
                    }
                }

                false
            }
        }
    }

    pub fn detach_node(&mut self, mut node: Shared<T>) {
        debug_assert!(self.has_node(node), "this node does not belong this list");

        unsafe { node.as_mut().detach() };
        self.length -= 1;

        if self.length == 0 {
            self.node = None;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::heap::{Alloc, Layout, System};
    use std::mem;

    struct Frame {
        next: Shared<Frame>,
        prev: Shared<Frame>,
        order: usize,
        is_alloc: bool,
    }

    impl Node<Frame> for Frame {
        fn as_ptr(&mut self) -> *mut Frame {
            self as *mut _
        }

        fn set_next(&mut self, s: Shared<Frame>) {
            self.next = s;
        }

        fn set_prev(&mut self, s: Shared<Frame>) {
            self.prev = s;
        }

        fn next(&self) -> &Frame {
            unsafe { self.next.as_ref() }
        }

        fn next_mut(&mut self) -> &mut Frame {
            unsafe { self.next.as_mut() }
        }

        fn prev(&self) -> &Frame {
            unsafe { self.prev.as_ref() }
        }

        fn prev_mut(&mut self) -> &mut Frame {
            unsafe { self.prev.as_mut() }
        }
    }

    fn allocate_nodes<T>(count: usize) -> *mut T {
        let type_size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let layout = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();

        ptr as _
    }

    #[test]
    fn test_usage() {
        static SIZE: usize = 1024;
        let nodes: *mut Frame = allocate_nodes(SIZE);
        let get_ith_frame = |i: usize| unsafe { &mut *nodes.offset(i as isize) as &mut Frame };

        for i in 0..SIZE {
            let f = get_ith_frame(i);
            f.order = i;
            f.is_alloc = false;
        }

        let frame = get_ith_frame(0);
        frame.init_link();
        assert_eq!(frame.length(), 1);

        for i in 1..SIZE {
            let f = get_ith_frame(i);
            let s = Shared::from(f);
            frame.insert_next(s);
            assert_eq!(frame.length(), 1 + i);
        }

        assert_eq!(frame.length(), SIZE);

        {
            // missing ?
            let f = get_ith_frame(10);
            f.detach();
            assert_eq!(frame.length(), SIZE - 1);
        }

        assert_eq!(frame.find(|_| false).is_none(), true);
        let r = frame.find(|n| n.order == 100);

        unsafe {
            assert_eq!(r.is_some(), true);
            assert_eq!(r.unwrap().as_ref().order, 100);
        }
    }
}
