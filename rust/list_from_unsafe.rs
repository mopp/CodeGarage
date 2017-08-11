#![feature(unique)]
#![feature(nonzero)]

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
            match unsafe {Unique::new(ptr)} {
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


fn main()
{
    const SIZE: usize = 1024;
    let mut memory_regions_source: [PtrNode<MemoryRegion>; SIZE];
    unsafe {
        let mut regions: [MemoryRegion; SIZE] = mem::uninitialized();

        let mut addr = 0x0000;
        for u in &mut regions[0..] {
            ptr::write(u, MemoryRegion(addr, true));
            addr += 0x1000;
        }

        memory_regions_source = mem::uninitialized();
        for i in 0..SIZE {
            let r = &mut regions[i];
            let u = &mut memory_regions_source[i];
            ptr::write(u, PtrNode::new(r as *mut _));
        }
    };

    let mut list: List<MemoryRegion> = List::new();
    assert_eq!(list.len(), 0);

    for n in memory_regions_source.as_mut() {
        list.push_front(n as *mut _);
    }
    assert_eq!(list.len(), SIZE);

    for i in 0..SIZE {
        let ptr = list.pop_front();
        assert_eq!(Some(&mut memory_regions_source[SIZE - i - 1] as *mut _), ptr);
    }

    assert_eq!(list.len(), 0);
}
