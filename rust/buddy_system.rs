extern crate list5;

use list5::LinkedList;
use list5::Node;

use std::default::Default;
use std::mem;
use std::ptr;


// 2^MAX_ORDER
const MAX_ORDER: usize = 16 + 1;


struct Frame {
    order: u8,
    is_free: bool
}


struct BuddyManager {
    lists: [LinkedList<Frame>; MAX_ORDER],
    count_free_frames: [usize; MAX_ORDER],
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
            let mut lists: [LinkedList<Frame>; MAX_ORDER] = mem::uninitialized();

            for l in lists.iter_mut() {
                ptr::write(l, LinkedList::new())
            }

            lists
        };

        BuddyManager {
            lists: lists,
            count_free_frames: [0; MAX_ORDER],
        }
    }
}


fn main()
{
}
