#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

use std::convert::{AsRef, AsMut};
use std::default::Default;
use std::ptr::{Unique, Shared};

pub type Link<T> = Shared<Node<T>>;


// pub struct Node<T> {
//     next: Option<Shared<Node<T>>>,
//     prev: Option<Shared<Node<T>>>,
//     element: T,
// }


pub trait Node<T> {
    fn next(&self) -> Option<&Node<T>>;
    fn prev(&self) -> Option<&Node<T>>;
    fn element(&self) -> &T;
    fn element_mut(&mut self) -> &mut T;
    fn set_next(&self, Option<&Node<T>>);
    fn set_prev(&self, Option<&Node<T>>);

    fn as_ref(&self) -> &T {
        self.element()
    }

    fn as_mut(&mut self) -> &T {
        self.element_mut()
    }

    fn length(&self) -> usize {
        panic!("not yet")
    }

    fn insert_next(&mut self, new_next: &mut Node<T>) {
        self.set_next(()
    }

    fn insert_prev(new_prev: &mut Node<T>) {
    }
}


fn main() {
}
