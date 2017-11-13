#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

use std::convert::{AsRef, AsMut};
use std::default::Default;
use std::ptr::{Unique, Shared};

pub type Link<T> = Shared<Node<T>>;


// pub struct Node<T> {
//     next: Option<Shared<Node<T>>>,
//     prev: Option<Shared<Node<T>>>,
//     element: T,
// }


pub trait Node<T> {
    fn init_link(&mut self);
    fn next(&self) -> Option<&Link<T>>;
    fn prev(&self) -> Option<&Link<T>>;
    fn next_mut(&mut self) -> Option<&mut Link<T>>;
    fn prev_mut(&mut self) -> Option<&mut Link<T>>;
    fn set_next(&mut self, Link<T>);

    // TODO: make this function.
    fn count_until(&self, target: &Link<T>, count: usize) -> usize {
        if let Some(next) = self.next() {
            if next.as_ptr() == target.as_ptr() {
                count
            } else {
                unsafe { next.as_ref().count_until(target, count + 1) }
            }
        } else {
            count
        }
    }

    fn length(&self) -> usize {
        match (self.next(), self.prev()) {
            (None, None)    => 0,
            (Some(_), None) => panic!("Invalid condition !"),
            (None, Some(_)) => panic!("Invalid condition !"),
            (Some(next), Some(prev)) => unsafe {
                next.as_ref().count_until(prev, 0)
            }
        }
    }

    fn insert_next(&mut self, new_next: *mut Node<T>) {
        if self.next().is_some() {
        } else {
            unsafe {
                self.set_next(Shared::new_unchecked(new_next))
            }
        }
    }

    fn insert_prev(&mut self, new_prev: &mut Node<T>) {
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::heap::{Alloc, System, Layout};
    use std::mem;
    use std::slice;

    struct Frame {
        next: Option<Link<Frame>>,
        prev: Option<Link<Frame>>,
        number: usize,
    }

    impl Frame {
        fn new(n: usize) -> Frame{
            Frame {
                next: None,
                prev: None,
                number: n,
            }
        }
    }

    impl Node<Frame> for Frame {
        fn init_link(&mut self) {
            self.next = None;
            self.prev = None;
            self.number = 0;
        }

        fn next(&self) -> Option<&Link<Frame>> {
            self.next.as_ref()
        }

        fn prev(&self) -> Option<&Link<Frame>> {
            self.prev.as_ref()
        }

        fn next_mut(&mut self) -> Option<&mut Link<Frame>> {
            self.next.as_mut()
        }

        fn prev_mut(&mut self) -> Option<&mut Link<Frame>> {
            self.prev.as_mut()
        }

        fn set_next(&mut self, new: Link<Frame>) {
            self.next = Some(new)
        }
    }

    fn allocate_nodes<T: Node<T>>(count: usize) -> *mut T {
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

        let frame1 = unsafe {&mut *nodes.offset(0) as &mut Frame};
        let frame2 = unsafe {&mut *nodes.offset(1) as &mut Frame};
        frame1.init_link();
        frame2.init_link();

        frame1.insert_next(frame2 as *mut _);
        frame2.init_link();

        assert_eq!(frame1.length(), 0);
    }
}
