#![cfg_attr(test, feature(allocator_api))]
#![feature(offset_to)]
#![feature(shared)]

extern crate list7;

use std::mem;
use std::ptr::Shared;
use std::ptr;

use list7::List;
use list7::Node;

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

                let target_frame = unsafe {
                    let ptr = frames.offset(index as isize);
                    let mut frame = Shared::new_unchecked(ptr);
                    frame.as_mut().order = order;
                    frame.as_mut().is_alloc = false;
                    frame
                };
                frame_lists[order].push_tail(target_frame);

                index += frame_count_in_order;
            }
        }

        BuddyManager {
            frame_ptr: frames,
            frame_count: frame_count,
            frame_lists: frame_lists,
        }
    }

    fn get_frame_index(&self, frame: Shared<Frame>) -> usize {
        match self.frame_ptr.offset_to(frame.as_ptr()) {
            Some(i) => i as usize,
            None => panic!("?"),
        }
    }

    fn get_buddy_frame(&self, frame: Shared<Frame>, order: usize) -> Option<Shared<Frame>> {
        let frame_addr = frame.as_ptr() as usize;
        let head_addr = self.frame_ptr as usize;
        let tail_addr = unsafe { self.frame_ptr.offset((self.frame_count - 1) as isize) as usize };

        debug_assert!(head_addr <= frame_addr, "Invalid frame is given");
        debug_assert!(frame_addr <= tail_addr, "Invalid frame is given");

        let is_in_valid_range = |addr: usize| (head_addr <= addr) && (addr <= tail_addr);

        if is_in_valid_range(frame_addr) == false {
            return None;
        }

        let buddy_index = self.get_frame_index(frame) ^ (1 << order);
        let buddy_addr = unsafe { self.frame_ptr.offset(buddy_index as isize) };

        if is_in_valid_range(buddy_addr as usize) == false {
            None
        } else {
            Some(unsafe { Shared::new_unchecked(buddy_addr) })
        }
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
                    unsafe {
                        frame.as_mut().order = request_order;
                        frame.as_mut().is_alloc = true;
                    };

                    // Push extra frames.
                    for i in request_order..order {
                        match self.get_buddy_frame(frame, i) {
                            Some(mut buddy_frame) => {
                                unsafe {
                                    buddy_frame.as_mut().order = i;
                                    buddy_frame.as_mut().is_alloc = false;
                                }
                                self.frame_lists[i].push_tail(buddy_frame);
                            }
                            None => unreachable!("why"),
                        }
                    }

                    return Some(frame);
                }
                Some(mut frame) => {
                    unsafe {
                        frame.as_mut().order = request_order;
                        frame.as_mut().is_alloc = true;
                    };
                    return Some(frame);
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
                Some(mut buddy_frame) => {
                    if unsafe { buddy_frame.as_ref() }.is_alloc {
                        continue;
                    }

                    self.frame_lists[order].detach_node(buddy_frame);

                    // Select frame which has smaller address.
                    if buddy_frame.as_ptr() < merged_frame.as_ptr() {
                        merged_frame = buddy_frame;
                    }
                    unsafe {
                        merged_frame.as_mut().order = order + 1;
                        merged_frame.as_mut().is_alloc = false;
                    };
                }
                _ => {
                    break;
                }
            }
        }

        let order = unsafe { merged_frame.as_ref().order };
        println!("free - {:?}", order);
        self.frame_lists[order].push_tail(merged_frame);
    }

    fn free_frame_count(&self) -> usize {
        self.frame_lists
            .iter()
            .enumerate()
            .fold(0, |acc, (order, frame_list)| {
                acc + (frame_list.length() * (1 << order))
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::heap::{Alloc, Layout, System};
    use std::mem;

    fn allocate_nodes<T>(count: usize) -> *mut T {
        let type_size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let layout = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();

        ptr as _
    }

    #[test]
    fn test_buddy_manager() {
        // 1,2,4,8,16,32,64
        static SIZE: usize = 1024 + (1 + 8);
        let frames: *mut Frame = allocate_nodes(SIZE);

        let mut bman = BuddyManager::new(frames, SIZE);
        assert_eq!(bman.frame_lists[10].length(), 1);
        assert_eq!(bman.frame_lists[3].length(), 1);
        assert_eq!(bman.frame_lists[0].length(), 1);
        assert_eq!(bman.free_frame_count(), SIZE);

        let frame1 = bman.alloc(0);
        assert_eq!(frame1.is_some(), true);
        assert_eq!(bman.frame_lists[0].length(), 0);
        assert_eq!(bman.free_frame_count(), SIZE - 1);

        let frame2 = bman.alloc(0);
        assert_eq!(frame2.is_some(), true);
        assert_eq!(bman.frame_lists[0].length(), 1);
        assert_eq!(bman.frame_lists[1].length(), 1);
        assert_eq!(bman.frame_lists[2].length(), 1);
        assert_eq!(bman.free_frame_count(), SIZE - 2);

        bman.free(frame1.unwrap());
        assert_eq!(bman.free_frame_count(), SIZE - 1);

        bman.free(frame2.unwrap());
        assert_eq!(bman.frame_lists[10].length(), 1);
        assert_eq!(bman.frame_lists[3].length(), 1);
        assert_eq!(bman.frame_lists[0].length(), 1);
        assert_eq!(bman.free_frame_count(), SIZE);

        let frame1 = bman.alloc(100);
        assert_eq!(frame1.is_none(), true);

        let frame1 = bman.alloc(1);
        assert_eq!(unsafe {frame1.unwrap().as_ref()}.order, 1);
    }
}
