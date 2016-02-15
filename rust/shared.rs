#![feature(shared)]

extern crate core;
use core::ptr::Shared;

struct A {
    tag: usize,
}

struct B {
    content: Shared<A>,
}


fn main()
{
    let mut obj_a = A {tag: 0};

    let obj_b = B {content: unsafe {Shared::new(&mut obj_a)}};

    let obj_a_shared = unsafe {&mut **obj_b.content};

    println!("{:?}", obj_a_shared.tag);
    println!("{:?}", obj_a.tag);

    obj_a.tag = 999;

    println!("{:?}", obj_a_shared.tag);
    println!("{:?}", obj_a.tag);

    obj_a_shared.tag = 100;
    let obj_a_shared_another = unsafe {&**obj_b.content};
    println!("{:?}", obj_a_shared_another.tag);
    println!("{:?}", obj_a_shared.tag);
    println!("{:?}", obj_a.tag);

    drop(obj_a);
    println!("{:?}", obj_a_shared_another.tag);
    println!("{:?}", obj_a_shared.tag);
}
