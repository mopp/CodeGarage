#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

use std::ptr::Shared;

pub trait Node<T: Node<T>> {
    fn as_ptr(&mut self) -> *mut T;
    fn as_ptr_n(&self) -> *const T;
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

    fn length(&mut self) -> usize {
        fn find_last<T>(current: &T, target: *const T, count: usize) -> usize {
            if current.as_ptr_n() == target {
                count
            } else {
                find_last(current.next(), target, count + 1)
            }
        }

        find_last(self.next(), self.prev().as_ptr_n(), 1)
    }

    fn insert_next(&mut self, mut new_next: Shared<T>) {
        if self.as_ptr() == new_next.as_ptr() {
            return;
        }

        unsafe {
            {
                let new_next = new_next.as_mut();
                new_next.set_next(self.as_shared());
                new_next.set_prev(self.next().into())
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

        fn as_ptr_n(&self) -> *const Frame {
            self as *const _
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

        fn prev_mut(&mut self) -> &mut Frame {
            unsafe {self.prev.as_mut()}
        }

        fn prev(&self) -> &Frame {
            unsafe {self.prev.as_ref()}
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
        for i in 1..SIZE  {
            let f = unsafe {&mut *nodes.offset(i as isize) as &mut Frame};
            f.init_link();
            frame.insert_next(Shared::from(f));
        }

        assert_eq!(frame.length(), SIZE);
    }
}
