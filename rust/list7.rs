#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

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
        unsafe {
            Shared::new_unchecked(self.as_ptr())
        }
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

    fn insert_next(&mut self, mut new_next: Shared<T>) {
        if self.as_ptr() == new_next.as_ptr() {
            return;
        }

        unsafe {
            {
                let new_next = new_next.as_mut();
                new_next.set_next(self.next().into());
                new_next.set_prev(self.as_shared())
            }
            self.next_mut().set_prev(new_next);
            self.set_next(new_next);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::heap::{Alloc, System, Layout};
    use std::mem;

    struct Frame {
        next: Shared<Frame>,
        prev: Shared<Frame>,
        number: usize,
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
            unsafe {self.next.as_ref()}
        }

        fn next_mut(&mut self) -> &mut Frame {
            unsafe {self.next.as_mut()}
        }

        fn prev(&self) -> &Frame {
            unsafe {self.prev.as_ref()}
        }

        fn prev_mut(&mut self) -> &mut Frame {
            unsafe {self.prev.as_mut()}
        }
    }

    fn allocate_nodes<T>(count: usize) -> *mut T {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();

        ptr as _
    }

    #[test]
    fn test_usage() {
        static SIZE: usize = 1024;
        let nodes: *mut Frame = allocate_nodes(SIZE);

        let frame = unsafe {&mut *nodes.offset(0) as &mut Frame};
        frame.init_link();
        assert_eq!(frame.length(), 1);

        for i in 1..SIZE  {
            let f = unsafe {&mut *nodes.offset(i as isize) as &mut Frame};
            let s = Shared::from(f);
            frame.insert_next(s);
            assert_eq!(frame.length(), 1 + i);
        }

        assert_eq!(frame.length(), SIZE);
    }
}
