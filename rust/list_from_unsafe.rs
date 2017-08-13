#![feature(unique)]
#![feature(nonzero)]
#![feature(allocator_api, heap_api)]

use std::mem;
use std::ptr::Unique;
use std::ptr;

extern crate core;
use core::nonzero::NonZero;
use core::nonzero::Zeroable;


#[derive(Debug)]
struct MemoryRegion(usize, bool);


struct PtrNode<T> {
    pointer: NonZero<*mut T>,
    next: Option<Unique<PtrNode<T>>>
}


impl<T> PtrNode<T> {
    fn new(v: *mut T) -> PtrNode<T>
    {
        match NonZero::new(v) {
            None     => panic!("Got a null !"),
            Some(nz) => {
                PtrNode {
                    pointer: nz,
                    next: None,
                }
            }
        }
    }

    unsafe fn as_ptr(&self) -> *mut T
    {
        self.pointer.get()
    }

    fn as_ref(&self) -> &T
    {
        unsafe { &*self.as_ptr() }
    }

    fn as_mut(&self) -> &mut T
    {
        unsafe { &mut *self.as_ptr() }
    }
}


unsafe impl<T> Zeroable for PtrNode<T> {
    fn is_zero(&self) -> bool
    {
        (&self as *const _).is_null()
    }
}


struct List<T> {
    head: Option<Unique<PtrNode<T>>>,
}


impl<T> List<T> {
    fn new() -> List<T>
    {
        List {
            head: None,
        }
    }

    fn push_front(&mut self, ptr: *mut PtrNode<T>)
    {
        let unique_ptr =
            match Unique::new(ptr) {
                None  => panic!("Got a null !"),
                Some(u)  => u
            };

        self.head =
            match self.head {
                None       => Some(unique_ptr),
                Some(next) => unsafe {
                    (*ptr).next = Some(next);
                    Some(unique_ptr)
                }
            }
    }

    fn push_back(&mut self, ptr: *mut PtrNode<T>)
    {
        fn find_last_node<T>(node: &mut Option<Unique<PtrNode<T>>>) -> &mut Option<Unique<PtrNode<T>>>
        {
            match *node {
                None            => node,
                Some(ref mut n) => find_last_node(&mut unsafe {n.as_mut()}.next)
            }
        }

        let last_ptr_node = find_last_node(&mut self.head);
        *last_ptr_node = Some(unsafe {Unique::new_unchecked(ptr)});
    }

    fn pop_front(&mut self) -> Option<*mut PtrNode<T>>
    {
        match self.head {
            None  => None,
            Some(mut n)  => {
                let ptr = unsafe {n.as_mut()};

                self.head = ptr.next;
                ptr.next = None;

                Some(ptr)
            }
        }
    }

    fn pop_back(&mut self) -> Option<*mut PtrNode<T>>
    {
        fn find_prev_last_node<T>(opt: &mut Option<Unique<PtrNode<T>>>, count: usize) -> &mut Option<Unique<PtrNode<T>>>
        {
            if count <= 2 {
                opt
            } else {
                find_prev_last_node(&mut unsafe {opt.as_mut().unwrap().as_mut()}.next, count - 1)
            }
        }

        let cnt = self.len();
        if cnt == 1 {
            let r = self.head.unwrap().as_ptr();
            self.head = None;
            return Some(r);
        }

        match *find_prev_last_node(&mut self.head, cnt) {
            None => None,
            Some(uniq)  => unsafe {
                let next_node = uniq.as_ptr();
                let r = (*next_node).next;
                (*next_node).next = None;
                Some(r.unwrap().as_ptr())
            }
        }
    }

    fn len(&self) -> usize
    {
        fn count<T>(node: &Option<Unique<PtrNode<T>>>, acc: usize) -> usize
        {
            match *node {
                None        => acc,
                Some(ref n) => count(&unsafe {n.as_ref()}.next, acc + 1)
            }
        }
        count(&self.head, 0)
    }

    fn is_empty(&self) -> bool
    {
        self.len() == 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, System, Layout};
    use std::slice;

    fn allocate_node_objs_for_test<'a, T>(count: usize) -> &'a mut [PtrNode<T>]
    {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr       = unsafe { System.alloc(layout) }.unwrap();
        let objs      = unsafe { slice::from_raw_parts_mut(ptr as *mut T, count) };

        let type_size = mem::size_of::<PtrNode<T>>();
        let align     = mem::align_of::<PtrNode<T>>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr       = unsafe { System.alloc(layout) }.unwrap();
        let nodes     = unsafe { slice::from_raw_parts_mut(ptr as *mut PtrNode<_>, count) };

        for i in 0..count {
            let n = PtrNode::new(&mut objs[i] as *mut _);
            unsafe { ptr::write(&mut nodes[i], n) };
        }

        nodes
    }

    struct Frame {
        order: u8,
        is_free: bool,
    }

    const MAX_ORDER: usize = 14 + 1;

    struct BuddyManager {
        frames: *mut PtrNode<Frame>,
        base_addr: usize,
        count_all_frames: usize,
        count_free_frames_in_group: [usize; MAX_ORDER],
        frame_group_lists: [List<Frame>; MAX_ORDER]
    }

    impl BuddyManager {
        fn new(frames: *mut PtrNode<Frame>, count_all_frames: usize) -> BuddyManager
        {
            let mut bman =
                BuddyManager {
                    frames: frames,
                    base_addr: 0,
                    count_all_frames: count_all_frames,
                    count_free_frames_in_group: [0; MAX_ORDER],
                    frame_group_lists: [List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new(), List::new()],
                };

            // Init all frames.
            let slice = unsafe { slice::from_raw_parts_mut(frames as *mut PtrNode<Frame>, count_all_frames) };

            let mut idx = 0;
            let mut count_rest_frames = count_all_frames;
            for order in (0..MAX_ORDER).rev() {
                if count_rest_frames == 0 {
                    break;
                }

                let list = &mut bman.frame_group_lists[order];
                let count_frames_in_group = 1 << order;

                let mut count_frame_blocks = 0;
                while count_frames_in_group <= count_rest_frames{
                    for node in slice[idx..(idx + count_frames_in_group)].iter_mut() {
                        {
                            let f = node.as_mut();
                            f.order = order as u8;
                            f.is_free = true;
                        }
                        list.push_back(node);
                    }

                    count_frame_blocks += 1;
                    idx                += count_frames_in_group;
                    count_rest_frames  -= count_frames_in_group;
                }
                bman.count_free_frames_in_group[order] = count_frame_blocks;
            }

            bman
        }

        fn alloc(&mut self, request_order: usize) -> Option<*mut PtrNode<Frame>>
        {
            for order in request_order..MAX_ORDER {
                if self.frame_group_lists[order].is_empty() {
                    continue;
                }

                // Found an enough frame and detach the frame.
                let allocate_node = self.frame_group_lists[order].pop_front().unwrap();
                self.count_free_frames_in_group[order] -= 1;

                {
                    let f = allocate_node.as_mut();
                    f.order = request_order;
                    f.is_free = false;
                }

                // If the frame that has larger order than requested order, remain frames of the frame should be inserted into other frame lists.
            }

            None
        }
    }

    #[test]
    fn test_usage()
    {
        const SIZE: usize = 1 + 1024;
        let frames = allocate_node_objs_for_test::<Frame>(SIZE);
        let mut bman = BuddyManager::new(&mut frames[0] as *mut _, SIZE);

        for i in 0..MAX_ORDER {
            let cnt = bman.count_free_frames_in_group[i];
            match i {
                0 | 10 => assert_eq!(cnt, 1),
                _  => assert_eq!(cnt, 0),
            }
        }

        let n = bman.alloc(0);
    }

    #[test]
    fn test_len()
    {
        let mut list = List::new();
        let memory_regions_source = allocate_node_objs_for_test::<MemoryRegion>(1024);

        let mut cnt = 0;
        for m in memory_regions_source.as_mut() {
            assert_eq!(list.len(), cnt);

            list.push_back(m);

            cnt += 1;
        }

        assert_eq!(list.len(), 1024);
    }

    #[test]
    fn test_push_front()
    {
        let mut list = List::new();
        let memory_regions_source = allocate_node_objs_for_test::<MemoryRegion>(1024);

        for m in memory_regions_source.as_mut() {
            list.push_front(m);
            assert_eq!(list.head.unwrap().as_ptr(), m as *mut _);
        }
    }

    #[test]
    fn test_push_back()
    {
        let mut list = List::new();
        let memory_regions_source = allocate_node_objs_for_test::<MemoryRegion>(1024);

        for m in memory_regions_source.as_mut() {
            list.push_back(m);
            assert_eq!(list.head.unwrap().as_ptr(), m as *mut _);
            list.pop_front();
        }
    }

    #[test]
    fn test_pop_back()
    {
        let mut list = List::new();
        let memory_regions_source = allocate_node_objs_for_test::<MemoryRegion>(1024);

        for m in memory_regions_source.as_mut() {
            list.push_front(m);
        }

        for m in memory_regions_source {
            assert_eq!(list.pop_back(), Some(m as *mut _));
        }
        assert_eq!(list.len(), 0);
    }
}
