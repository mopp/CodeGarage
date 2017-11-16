#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

use std::ptr::Shared;

pub trait Node {
    fn next(&self) -> &Shared<Self>;
    fn prev(&self) -> &Shared<Self>;
    fn next_mut(&mut self) -> &mut Shared<Self>;
    fn prev_mut(&mut self) -> &mut Shared<Self>;

    fn is_self(&self, target: &Shared<Self>) -> bool {
        let shared: Shared<Self> = self.into();
        shared.as_ptr() == target.as_ptr()
    }

    fn init_link(&mut self) {
        *self.next_mut() = self.into();
        *self.prev_mut() = self.into();
    }

    fn count_until(&self, target: &Shared<Self>, count: usize) -> usize {
        if self.is_self(target) {
            count
        } else {
            unsafe {
                self.next().as_ref().count_until(target, count + 1)
            }
        }
    }

    fn length(&self) -> usize {
        self.count_until(self.prev(), 1)
    }

    fn insert_next(&mut self, mut new_next: Shared<Self>) {
        if self.is_self(&new_next) {
            return;
        }

        unsafe {
            {
                let new_next = new_next.as_mut();
                *new_next.prev_mut() = self.into();
                *new_next.next_mut() = *self.next();
            }
            let next_mut = self.next_mut();
            *next_mut.as_mut().prev_mut() = new_next;
            *next_mut = new_next;
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
