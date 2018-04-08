#![cfg_attr(test, feature(allocator_api))]
#![feature(offset_to)]
#![feature(ptr_internals)]
#![feature(unique)]

extern crate list5;

use list5::LinkedList;
use list5::Node;
use std::mem;
use std::ptr::NonNull;
use std::ptr::Unique;
use std::ptr;

const MAX_ORDER: usize = 15;

/// It manages type T in 2^N (where N is order)
/// it is better to make T phantom type ?
/// in the case it just manages the indices.
// trait BuddyManager<T> {
//     fn new(count_object: usize) -> BuddyManager<T> where Self: Sized;
//     fn allocate(&mut self, usize) -> Option<T>;
//     fn free(&mut self, T);
// }

struct BuddyObject {
    order: usize
}

struct BuddyManager {
    nodes: *mut Node<BuddyObject>,
    count_frames: usize,
    lists: [LinkedList<usize>; MAX_ORDER],
}

impl BuddyManager {
    fn new(nodes: *mut Node<BuddyObject>, count: usize) -> BuddyManager {
        let lists = unsafe {
            let mut lists: [LinkedList<usize>; MAX_ORDER] = mem::uninitialized();

            for l in lists.iter_mut() {
                ptr::write(l, LinkedList::new())
            }

            lists
        };

        let mut bman = BuddyManager {
            nodes: nodes,
            count_frames: count,
            lists: lists,
        };

        bman
    }

    fn allocate(&mut self, request_order: usize) -> Option<usize> {
    }

    fn free(&mut self, index: usize) {
        let buddy_obj = self.nodes.offset(index as isize);
        let order = buddy_obj.order;
    }
}


// /// keep target objects to manage.
trait ObjectMapper<T, U, E> {
    fn new(*mut U, usize) -> Result<Self, E> where Self: Sized;
    fn to(&self, T) -> Result<NonNull<U>, E>;
    fn from(&self, NonNull<U>) -> Result<T, E>;
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, Layout, System};
    use std::mem;
    use std::slice;
    use std::ptr;

    struct Frame {
        order: u8,
        is_used: bool
    }

    struct FrameMapper {
        ptr: Unique<Frame>,
        count: usize
    }

    impl ObjectMapper<usize, Frame, String> for FrameMapper {
        fn new(raw_ptr: *mut Frame, count: usize) -> Result<Self, String> where Self: Sized {
            if count == 0 {
                return Err("count is zero".to_string());
            }

            Unique::new(raw_ptr)
                .ok_or("ERROR: null pointer".to_string())
                .map(|ptr| {
                    FrameMapper {
                        ptr: ptr,
                        count: count
                    }
                })
        }

        fn to(&self, i: usize) -> Result<NonNull<Frame>, String> {
            unsafe {
                let ptr = self.ptr.as_ptr().offset(i as isize);
                NonNull::new(ptr).ok_or("TODO".to_string())
            }
        }

        fn from(&self, frame: NonNull<Frame>) -> Result<usize, String> {
            self.ptr
                .as_ptr()
                .offset_to(frame.as_ptr())
                .map(|i| i as usize)
                .ok_or("TODO".to_string())
        }
    }

    fn allocate_objs<'a, T>(count: usize) -> *mut T {
        let type_size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let layout = Layout::from_size_align(count * type_size, align).unwrap();

        let ptr = unsafe { System.alloc(layout) }.unwrap();

        ptr as *mut T
    }

    #[test]
    fn mapper() {
        const SIZE: usize = 64;
        let frames: *mut Frame  = allocate_objs(SIZE);

        let mapper = FrameMapper::new(frames, SIZE);
        assert_eq!(true, mapper.is_ok());
        let mapper = mapper.unwrap();

        let f = mapper.to(0);
        assert_eq!(true, f.is_ok());
        let f = f.unwrap();
        assert_eq!(true, ptr::eq(f.as_ptr(), frames));

        let i = mapper.from(f);
        assert_eq!(true, i.is_ok());
        let i = i.unwrap();
        assert_eq!(0, i);
    }

    #[test]
    fn new() {
        const SIZE: usize = 64;
        let frames: *mut Frame = allocate_objs(SIZE);
        let nodes: *mut Node<usize> = allocate_objs(SIZE);

        let bman = BuddyManager::new(nodes, SIZE);

        f1 = bman.allocate(1);

        list.push(f1);

        bman.free(f1);
    }
}
