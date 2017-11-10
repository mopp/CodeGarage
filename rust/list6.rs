#![feature(shared)]
#![feature(unique)]
#![cfg_attr(test, feature(allocator_api))]

use std::convert::{AsRef, AsMut};
use std::default::Default;
use std::ptr::{Unique, Shared};


pub struct Node<T: Default> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    element: T,
}

trait Node<T> {
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &T;
    fn length(&self) -> usize;
}
