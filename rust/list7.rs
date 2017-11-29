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
}

impl<T: Node<T>> List<T> {
    pub fn new() -> List<T> {
        List { node: None }
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

    const MAX_ORDER: usize = 15;

    struct BuddyManager {
        frame_ptr: *mut Frame,
        frame_count: usize,
        frame_lists: [List<Frame>; MAX_ORDER],
        free_frame_counts: [usize; MAX_ORDER],
    }

    impl BuddyManager {
        pub fn new(frames: *mut Frame, frame_count: usize) -> BuddyManager {
            let mut frame_lists = unsafe {
                let mut lists: [List<Frame>; MAX_ORDER] = mem::uninitialized();

                for l in lists.iter_mut() {
                    ptr::write(l, List::new())
                }

                lists
            };

            let mut free_frame_counts = [0; MAX_ORDER];

            // Init all frames.
            for i in 0..frame_count {
                let f = unsafe { &mut *frames.offset(i as isize) as &mut Frame };
                f.init_link();
            }

            let mut index = 0usize;
            for order in (0..MAX_ORDER).rev() {
                let frame_count_in_order = 1 << order;
                loop {
                    if (frame_count - index) < frame_count_in_order {
                        break;
                    }

                    let target_frame =
                        unsafe { Shared::new_unchecked(frames.offset(index as isize)) };
                    frame_lists[order].push_tail(target_frame);

                    free_frame_counts[order] += 1;
                    index += frame_count_in_order;
                }
            }

            BuddyManager {
                frame_ptr: frames,
                frame_count: frame_count,
                frame_lists: frame_lists,
                free_frame_counts: free_frame_counts,
            }
        }

        fn get_buddy_frame(&self, f: Shared<Frame>, order: usize) -> Option<Shared<Frame>> {
            let frame_addr = f.as_ptr() as usize;
            let head_addr = self.frame_ptr as usize;
            let tail_addr = unsafe { self.frame_ptr.offset(self.frame_count as isize) as usize };

            debug_assert!(head_addr < frame_addr, "Invalid frame is given");
            debug_assert!(frame_addr < tail_addr, "Invalid frame is given");

            if (frame_addr <= head_addr) || (tail_addr <= frame_addr) {
                return None;
            }

            let buddy_ptr = frame_addr ^ (1 << order);
            Some(unsafe { Shared::new_unchecked(buddy_ptr as *mut _) })
        }

        pub fn alloc(&mut self, request_order: usize) -> Option<Shared<Frame>> {
            if MAX_ORDER <= request_order {
                return None;
            }

            // find last set instruction makes it more accelerate ?
            // 0001 1000
            // fls(map >> request_order) ?
            for order in request_order..MAX_ORDER {
                match self.frame_lists[order].pop_head() {
                    None => {
                        continue;
                    }
                    Some(mut frame) if request_order < order => {
                        self.free_frame_counts[order] -= 1;

                        unsafe { frame.as_mut().order = request_order };

                        // Push extra frames.
                        for i in request_order..order {
                            match self.get_buddy_frame(frame, i) {
                                Some(mut buddy_frame) => {
                                    unsafe {
                                        buddy_frame.as_mut().order = i;
                                    }
                                    self.frame_lists[i].push_tail(buddy_frame);
                                    self.free_frame_counts[i] += 1;
                                }
                                None => unreachable!("why"),
                            }
                        }

                        return Some(frame);
                    }
                    frame => {
                        self.free_frame_counts[order] -= 1;
                        return frame;
                    }
                }
            }

            None
        }

        pub fn free(&mut self, frame: Shared<Frame>) {
            let order = unsafe { frame.as_ref().order };

            let mut merged_frame = frame;
            for order in order..MAX_ORDER {
                match self.get_buddy_frame(merged_frame, order) {
                    Some(mut shared_buddy_frame) => {
                        if unsafe {shared_buddy_frame.as_ref()}.is_alloc {
                            continue;
                        }

                        {
                            let buddy_frame = unsafe{ shared_buddy_frame.as_mut() };
                            self.free_frame_counts[buddy_frame.order] -= 1;
                            buddy_frame.detach();
                            if self.free_frame_counts[buddy_frame.order] == 0 {
                                self.frame_lists[buddy_frame.order].node = None;
                            }
                        }

                        let buddy_frame_addr = shared_buddy_frame.as_ptr() as usize;
                        let merged_frame_addr = merged_frame.as_ptr() as usize;
                        // Select frame which has smaller address.
                        if buddy_frame_addr < merged_frame_addr {
                            merged_frame = shared_buddy_frame;
                        }
                    }
                    _ => break
                }
            }

            let order = unsafe { merged_frame.as_ref().order };
            self.frame_lists[order].push_tail(merged_frame);
            self.free_frame_counts[order] += 1;
        }

        fn free_frame_count(&self) -> usize {
            self.free_frame_counts
                .iter()
                .enumerate()
                .fold(0, |acc, (order, &x)| acc + (x * (1 << order)))
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

    #[test]
    fn test_buddy_manager() {
        // 1,2,4,8,16,32,64
        static SIZE: usize = 1024 + (1 + 8);
        println!("{}", SIZE);
        let frames: *mut Frame = allocate_nodes(SIZE);

        let mut bman = BuddyManager::new(frames, SIZE);
        assert_eq!(bman.free_frame_counts[10], 1);
        assert_eq!(bman.free_frame_counts[3], 1);
        assert_eq!(bman.free_frame_counts[0], 1);
        assert_eq!(bman.free_frame_count(), SIZE);

        assert_eq!(bman.alloc(0).is_some(), true);
        assert_eq!(bman.free_frame_counts[0], 0);
        assert_eq!(bman.free_frame_count(), SIZE - 1);

        assert_eq!(bman.alloc(0).is_some(), true);
        assert_eq!(bman.free_frame_counts[0], 1);
        assert_eq!(bman.free_frame_counts[1], 1);
        assert_eq!(bman.free_frame_counts[2], 1);
        assert_eq!(bman.free_frame_count(), SIZE - 2);
    }
}
