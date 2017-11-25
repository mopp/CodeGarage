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

    fn is_alone(&self) -> bool {
        let prev = self.prev();
        let next = self.next();
        let current = self.next().prev();

        ptr::eq(current as _, prev as _) && ptr::eq(current as _, next as _)
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
}

impl<T: Node<T>> List<T> {
    pub fn new() -> List<T> {
        List {
            node: None,
        }
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
    }

    pub fn push_head(&mut self, new_node: Shared<T>) {
        self.push(new_node, true);
    }

    pub fn push_tail(&mut self, new_node: Shared<T>) {
        self.push(new_node, false);
    }

    fn pop(&mut self, is_next: bool) -> Option<Shared<T>> {
        self.node
            .map(|mut node| {
                if unsafe {node.as_ref().is_alone()} {
                    self.node = None;
                } else {
                    let node = unsafe {node.as_mut()};
                    self.node =
                        Some(match is_next {
                            true => node.next_mut(),
                            false => node.prev_mut(),
                        }.as_shared());
                    node.detach();
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
        frame_lists: [List<Frame>; MAX_ORDER],
        frame_counts: [usize; MAX_ORDER],
    }

    impl BuddyManager {
        pub fn new(frames: *mut Frame, frame_count: usize) -> BuddyManager {
            let mut frame_lists: [List<Frame>; MAX_ORDER] = unsafe {mem::uninitialized()};
            for f in frame_lists.iter_mut() {
                *f = List::new();
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
                    frame_lists[order].push_tail(target_frame);

                    frame_counts[order] += 1;
                    index += frame_count_in_order;
                }
            }

            BuddyManager {
                frame_lists: frame_lists,
                frame_counts: frame_counts,
            }
        }

        fn get_buddy_frame(&self, f: Shared<Frame>, order: usize) -> Shared<Frame> {
            // TODO: check boundary.
            let buddy_ptr = (f.as_ptr() as usize) ^ (1 << order);
            unsafe { Shared::new_unchecked(buddy_ptr as _) }
        }

        pub fn alloc(&mut self, request_order: usize) -> Option<Shared<Frame>> {
            // find last set instruction makes it more accelerate ?
            // 0001 1000
            // fls(map >> request_order) ?
            for order in request_order..MAX_ORDER {
                match self.frame_lists[order].pop_head() {
                    None => {
                        continue;
                    },
                    Some(mut frame) if request_order < order => {
                        self.frame_counts[order] -= 1;

                        unsafe {
                            frame.as_mut().order = request_order
                        };

                        // Push extra frames.
                        for i in request_order..order {
                            unsafe {
                                let mut buddy_frame = self.get_buddy_frame(frame, i);
                                buddy_frame.as_mut().order = i;
                                self.frame_lists[i].push_tail(buddy_frame);
                                self.frame_counts[i] += 1;
                            }
                        }

                        return Some(frame);
                    },
                    frame => {
                        self.frame_counts[order] -= 1;
                        return frame;
                    },
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

        let mut bman = BuddyManager::new(frames, SIZE);
        assert_eq!(bman.frame_counts[10], 1);
        assert_eq!(bman.frame_counts[3], 1);
        assert_eq!(bman.frame_counts[0], 1);

        assert_eq!(bman.alloc(0).is_some(), true);
        assert_eq!(bman.frame_counts[0], 0);

        assert_eq!(bman.alloc(0).is_some(), true);
        assert_eq!(bman.frame_counts[0], 1);
        assert_eq!(bman.frame_counts[1], 1);
        assert_eq!(bman.frame_counts[2], 1);
    }
}
