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

    fn detach(&mut self) {
        let prev = self.prev().into();
        let next = self.next().into();
        self.next_mut().set_prev(prev);
        self.prev_mut().set_next(next);

        self.init_link();
    }

    fn find<F>(&mut self, f: F) -> Option<Shared<T>> where F: Fn(&T) -> bool {
        let tail = self.prev_mut().as_shared();
        let mut current = self.next_mut().prev_mut().as_shared();

        while ptr::eq(current.as_ptr(), tail.as_ptr()) == false {
            if f(unsafe {current.as_ref()}) {
                return Some(current);
            }

            unsafe {
                current = current.as_mut().next_mut().as_shared();
            }
        }

        None
    }

    fn insert_next(&mut self, mut new_next: Shared<T>) {
        if self.as_ptr() == new_next.as_ptr() {
            return;
        }

        {
            let new_next = unsafe { new_next.as_mut() };
            new_next.set_next(self.next().into());
            new_next.set_prev(self.as_shared())
        }
        self.next_mut().set_prev(new_next);
        self.set_next(new_next);
    }

    fn insert_prev(&mut self, mut new_prev: Shared<T>) {
        if self.as_ptr() == new_prev.as_ptr() {
            return;
        }

        {
            let new_prev = unsafe { new_prev.as_mut() };
            new_prev.set_next(self.as_shared());
            new_prev.set_prev(self.prev().into());
        }
        self.prev_mut().set_next(new_prev);
        self.set_prev(new_prev);
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
        order: usize,
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

    const MAX_ORDER: usize = 15;

    struct BuddyManager {
        frame_lists: [Option<Shared<Frame>>; MAX_ORDER],
        frame_counts: [usize; MAX_ORDER],
    }

    impl BuddyManager {
        pub fn new(frames: *mut Frame, frame_count: usize) -> BuddyManager {
            let mut frame_lists: [Option<Shared<Frame>>; MAX_ORDER] = unsafe {mem::uninitialized()};
            for f in frame_lists.iter_mut() {
                *f = None;
            }
            let mut frame_counts = [0; MAX_ORDER];

            // Init all frames.
            for i in 0..frame_count {
                let f = unsafe {&mut *frames.offset(i as isize) as &mut Frame};
                f.init_link();
            }

            let mut index = 0usize;
            for order in (0..MAX_ORDER).rev() {
                let frame_count_in_order = 1 << order;
                loop {
                    if (frame_count - index) < frame_count_in_order {
                        break;
                    }

                    let target_frame = unsafe {Shared::new_unchecked(frames.offset(index as isize))};
                    if let Some(mut f) = frame_lists[order] {
                        unsafe { f.as_mut().insert_next(target_frame); }
                    } else  {
                        // Set head.
                        frame_lists[order] = Some(target_frame);
                    }

                    frame_counts[order] += 1;
                    index += frame_count_in_order;
                }
            }

            BuddyManager {
                frame_lists: frame_lists,
                frame_counts: frame_counts,
            }
        }

        pub fn alloc(&mut self, request_order: usize) -> Option<Shared<Frame>> {
            for order in request_order..MAX_ORDER {
                let count = self.frame_counts[order];

                if count == 0 {
                    continue;
                } else if count == 1 {
                    let Some(mut frames) = self.frame_lists[order];
                    self.frame_lists[order] = None;

                    if request_order < order {
                        // Push back extra frames.
                        for i in (request_order..order) {
                            unsafe {
                                let ptr = frames.as_ptr();
                                let buddy_ptr = ptr ^ (1 << i);
                                let buddy_frame = Shared::new_unchecked(buddy_ptr);
                                self.frame_lists[i].insert_prev(buddy_frame);
                                self.frame_counts[i] += 1;
                            }
                        }
                    }

                    return Some(frames);
                } else {
                }

                if let Some(mut frames) = self.frame_lists[order] {
                    if self.frame_counts[order] == 1 {
                        self.frame_lists[order] = None;
                        return Some(frames);
                    } else {
                        unsafe { self.frame_lists[order] = Some(frames.as_mut().next_mut().as_shared()) };
                        return Some(frames);
                    }
                } else {
                    continue;
                }
            }

            None
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
        let get_ith_frame = |i: usize| unsafe {
            &mut *nodes.offset(i as isize) as &mut Frame
        };

        for i in 0..SIZE {
            let f = get_ith_frame(i);
            f.order = i;
        }

        let frame = get_ith_frame(0);
        frame.init_link();
        assert_eq!(frame.length(), 1);

        for i in 1..SIZE  {
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

    #[test]
    fn test_buddy_manager() {
        // 1,2,4,8,16,32,64
        static SIZE: usize = 1024 + (1 + 8);
        println!("{}", SIZE);
        let frames: *mut Frame = allocate_nodes(SIZE);

        let bman = BuddyManager::new(frames, SIZE);
        assert_eq!(bman.frame_counts[10], 1);
        assert_eq!(bman.frame_counts[3], 1);
        assert_eq!(bman.frame_counts[0], 1);
    }
}
