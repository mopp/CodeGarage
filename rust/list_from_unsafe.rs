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

        let mut last_ptr_node = find_last_node(&mut self.head);
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
                let mut next_node = uniq.as_ptr();
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
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, System, Layout};
    use std::slice;

    fn allocate_objs_for_test<'a>(count: usize) -> &'a mut [PtrNode<MemoryRegion>]
    {
        let layout = Layout::from_size_align(count * mem::size_of::<MemoryRegion>(), mem::align_of::<MemoryRegion>()).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();
        let mut regions: &mut [MemoryRegion] = unsafe {slice::from_raw_parts_mut(ptr as *mut MemoryRegion, count)};

        let mut addr = 0x0000;
        for u in &mut regions[0..] {
            let r = MemoryRegion(addr, true);
            unsafe { ptr::write(u, r) ;}
            addr += 0x1000;
        }

        let layout = Layout::from_size_align(count * mem::size_of::<PtrNode<MemoryRegion>>(), mem::align_of::<PtrNode<MemoryRegion>>()).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();
        let mut memory_regions_source = unsafe {slice::from_raw_parts_mut(ptr as *mut PtrNode<MemoryRegion>, count)};
        for i in 0..count {
            let u = &mut memory_regions_source[i];
            let n = PtrNode::new(&mut regions[i] as *mut _);
            unsafe {ptr::write(u, n)};
        }

        memory_regions_source
    }


    #[test]
    fn test_len()
    {
        let mut list = List::new();
        let memory_regions_source = allocate_objs_for_test(1024);

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
        let memory_regions_source = allocate_objs_for_test(1024);

        for m in memory_regions_source.as_mut() {
            list.push_front(m);
            assert_eq!(list.head.unwrap().as_ptr(), m as *mut _);
        }
    }

    #[test]
    fn test_push_back()
    {
        let mut list = List::new();
        let memory_regions_source = allocate_objs_for_test(1024);

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
        let memory_regions_source = allocate_objs_for_test(1024);

        for m in memory_regions_source.as_mut() {
            list.push_front(m);
        }

        for m in memory_regions_source {
            assert_eq!(list.pop_back(), Some(m as *mut _));
        }
        assert_eq!(list.len(), 0);
    }
}
