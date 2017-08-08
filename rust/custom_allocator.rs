#![feature(global_allocator, allocator_api, heap_api)]

use std::heap::{Alloc, System, Layout, AllocErr};

struct MyAllocator;

unsafe impl<'a> Alloc for &'a MyAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        println!("alloc");
        System.alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        println!("dealloc");
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn main() {
    // This `Vec` will allocate memory through `GLOBAL` above
    let mut v = Vec::new();
    v.push(1);
}
