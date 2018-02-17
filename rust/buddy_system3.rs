#![cfg_attr(test, feature(allocator_api))]
#![feature(offset_to)]
#![feature(unique)]

extern crate list5;

use std::marker::PhantomData;
use list5::LinkedList;
use list5::Node;

const MAX_ORDER: usize = 15;

/// It manages type T in 2^N (where N is order)
/// it is better to make T phantom type ?
/// in the case it just manages the indices.
// trait BuddyManager<T> {
//     fn new(count_object: usize) -> BuddyManager<T> where Self: Sized;
//     fn allocate(&mut self, usize) -> Option<T>;
//     fn free(&mut self, T);
// }

struct BuddyManager<T> {
    used_flags: [bool]
}

/// keep target objects to manage.
trait ObjectMapper<T, U> {
    fn new(*mut U) -> ObjectMapper<T, U> where Self: Sized;
    fn to(&mut self, T) -> U;
    fn from(&mut self, U) -> T;
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
