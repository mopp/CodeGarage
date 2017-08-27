#![cfg_attr(test, feature(allocator_api))]

extern crate list5;

use list5::LinkedList;
use list5::Node;

use std::default::Default;
use std::mem;
use std::ptr;


// 2^MAX_ORDER
const MAX_ORDER: u8 = 16 + 1;


struct Frame {
    order: u8,
    is_free: bool
}


struct BuddyManager {
    lists: [LinkedList<Frame>; MAX_ORDER as usize],
    count_free_frames: [usize; MAX_ORDER as usize],
}


impl Default for Frame {
    fn default() -> Frame
    {
        Frame {
            order: 0,
            is_free: true,
        }
    }
}


impl BuddyManager {
    fn new() -> BuddyManager
    {
        let mut lists = unsafe {
            let mut lists: [LinkedList<Frame>; MAX_ORDER as usize] = mem::uninitialized();

            for l in lists.iter_mut() {
                ptr::write(l, LinkedList::new())
            }

            lists
        };

        BuddyManager {
            lists: lists,
            count_free_frames: [0; MAX_ORDER as usize],
        }
    }

    fn supply_frame_nodes(&mut self, nodes: *mut Node<Frame>, count: usize)
    {
        let mut count_rest_frames = count;
        let mut current_node_ptr  = nodes;

        for (order, list) in self.lists.iter_mut().enumerate().rev() {
            let count_frames_in_list = 1usize << order;
            while (count_rest_frames != 0) && (count_frames_in_list <= count_rest_frames) {
                list.push_back(current_node_ptr);
                self.count_free_frames[order] += 1;

                current_node_ptr = unsafe { current_node_ptr.offset(count_frames_in_list as isize) };
                count_rest_frames -= count_frames_in_list;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, System, Layout};
    use std::mem;
    use std::slice;

    fn allocate_node_objs<'a, T>(count: usize) -> &'a mut [T] where T: Default
    {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr       = unsafe { System.alloc(layout) }.unwrap();
        unsafe { slice::from_raw_parts_mut(ptr as *mut T, count) }
    }

    #[test]
    fn init_buddy_manager()
    {
        let bman = BuddyManager::new();
    }

    #[test]
    fn supply_frame_nodes()
    {
        let count = 1024;
        let nodes = allocate_node_objs::<Node<Frame>>(count);
        let mut bman = BuddyManager::new();
        bman.supply_frame_nodes(&mut nodes[0] as *mut _, count);
        assert_eq!(bman.count_free_frames[10], 1);
    }
}
