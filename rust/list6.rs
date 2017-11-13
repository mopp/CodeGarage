#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

use std::ptr::Shared;

pub trait Node {
    fn next(&self) -> &Shared<Self>;
    fn prev(&self) -> &Shared<Self>;
    fn next_mut(&mut self) -> &mut Shared<Self>;
    fn prev_mut(&mut self) -> &mut Shared<Self>;

    fn init_link(&mut self) {
        *self.next_mut() = self.into();
        *self.prev_mut() = self.into();
    }

    fn count_until(&self, target: &Shared<Self>, count: usize) -> usize {
        if self.next().as_ptr() == target.as_ptr() {
            count
        } else {
            unsafe {
                self.next().as_ref().count_until(target, count + 1)
            }
        }
    }

    fn length(&self) -> usize {
        self.count_until(&Shared::from(self), 1)
    }

    fn insert_next(&mut self, mut new_next: Shared<Self>) {
        if self.next().as_ptr() == new_next.as_ptr() {
            return;
        }

        unsafe {
            *new_next.as_mut().prev_mut() = self.into();
            *new_next.as_mut().next_mut() = *self.next();
            *self.next_mut().as_mut().prev_mut() = new_next;
            *self.next_mut() = new_next;
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

    impl Node for Frame {
        fn next(&self) -> &Shared<Self> {
            &self.next
        }

        fn prev(&self) -> &Shared<Self> {
            &self.prev
        }

        fn next_mut(&mut self) -> &mut Shared<Self> {
            &mut self.next
        }

        fn prev_mut(&mut self) -> &mut Shared<Self> {
            &mut self.prev
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

        let frame1 = unsafe {&mut *nodes.offset(0) as &mut Frame};
        let frame2 = unsafe {&mut *nodes.offset(1) as &mut Frame};
        let frame3 = unsafe {&mut *nodes.offset(2) as &mut Frame};
        frame1.init_link();
        frame2.init_link();
        frame3.init_link();

        assert_eq!(frame1.length(), 1);

        frame1.insert_next(Shared::from(frame2));
        frame1.insert_next(Shared::from(frame3));
        assert_eq!(frame1.length(), 3);
    }
}
