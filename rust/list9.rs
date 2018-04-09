#![feature(unique)]
#![feature(ptr_internals)]

use std::ptr::{NonNull, Unique};
use std::marker::PhantomData;

type Link<T> = NonNull<Anchor<T>>;

#[repr(C)]
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_head(&mut self, node: Link<T>) {
        self.head = Some(node);
    }

    pub fn pop_head(&mut self) -> Option<Link<T>> {
        self.head
    }
}

#[repr(C)]
#[derive(Debug)]
struct Anchor<T> {
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
    phantom: PhantomData<T>,
}

impl<T> Anchor<T> {
    pub fn new() -> Anchor<T> {
        Anchor {
            next: None,
            prev: None,
            phantom: PhantomData
        }
    }

    unsafe fn restore(&mut self) -> &mut T {
        std::mem::transmute::<&mut Self, &mut T>(self)
    }
}

#[repr(C)]
#[derive(Debug)]
struct SampleNode {
    anchor: Anchor<SampleNode>,
    order: usize
}

impl SampleNode {
    pub fn new() -> SampleNode {
        SampleNode {
            anchor: Anchor::new(),
            order: 0
        }
    }
}

fn main() {
    let list = LinkedList::<SampleNode>::new();
    list.push_head()
    // let node1 = SampleNode::new();
    // let node2 = SampleNode::new();
    //
    // list.push_head(node1.anchor.to_nonnull);
    // list.push_head(node2.anchor.to_nonnull);
    //
    // let nonnull_anchor = list.pop_head();
    // let node1_1 = nonnull_anchor.as_ptr()

    // let addr = 0x1312312;
    // let obj: *mut SampleNode = addr as *mut _;
    // let nonnull = NonNull::new(obj);
    // println!("{:?}", nonnull.as_ref());
    // let n = nonnull.unwrap();
    // println!("{:?}", unsafe {n.as_ref()});
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
    }
}
