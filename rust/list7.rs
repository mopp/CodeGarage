#![cfg_attr(test, feature(allocator_api))]
#![feature(offset_to)]
#![feature(ptr_internals)]
#![feature(unique)]
#![no_std]

#![crate_type = "rlib"]
#![crate_name = "list7"]

// use core::ptr;
use core::ptr::{NonNull, Unique};

pub trait Node<T: Node<T>> {
    fn set_next(&mut self, NonNull<Self>);
    fn set_prev(&mut self, NonNull<Self>);
    fn next(&self) -> Option<NonNull<Self>>;
    fn prev(&self) -> Option<NonNull<Self>>;

    fn init_link(&mut self) {
        let ptr = unsafe { NonNull::new_unchecked(self as *mut _) };
        self.set_next(ptr);
        self.set_prev(ptr);
    }

    fn count(&self) -> usize {
        let mut count = 1;
        let mut node;

        // Count the nodes to forward.
        node = self.next();
        loop {
            if let Some(next) = node {
                node = unsafe { next.as_ref().next() };
                count += 1;
            } else {
                break;
            }
        }

        // Count the nodes to backward.
        node = self.prev();
        loop {
            if let Some(prev) = node {
                node = unsafe { prev.as_ref().prev() };
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    // fn detach(&mut self) {
    //     let prev = self.prev().into();
    //     let next = self.next().into();
    //     self.next_mut().set_prev(prev);
    //     self.prev_mut().set_next(next);
    //
    //     self.init_link();
    // }
    //
    // fn find<F>(&mut self, f: F) -> Option<NonNull<T>> where F: Fn(&T) -> bool {
    //     let tail = self.prev_mut().as_shared();
    //     let mut current = self.next_mut().prev_mut().as_shared();
    //
    //     while ptr::eq(current.as_ptr(), tail.as_ptr()) == false {
    //         if f(unsafe { current.as_ref() }) {
    //             return Some(current);
    //         }
    //
    //         unsafe {
    //             current = current.as_mut().next_mut().as_shared();
    //         }
    //     }
    //
    //     None
    // }
    //
    // fn insert_next(&mut self, mut new_next: NonNull<T>) {
    //     {
    //         let new_next = unsafe { new_next.as_mut() };
    //         new_next.set_next(self.next().into());
    //         new_next.set_prev(self.as_shared())
    //     }
    //
    //     self.next_mut().set_prev(new_next);
    //     self.set_next(new_next);
    // }
    //
    // fn insert_prev(&mut self, mut new_prev: NonNull<T>) {
    //     {
    //         let new_prev = unsafe { new_prev.as_mut() };
    //         new_prev.set_next(self.as_shared());
    //         new_prev.set_prev(self.prev().into());
    //     }
    //
    //     self.prev_mut().set_next(new_prev);
    //     self.set_prev(new_prev);
    // }
}

pub struct LinkedList<T: Node<T>> {
    head: Option<NonNull<T>>,
    tail: Option<NonNull<T>>,
}

impl<T: Node<T>> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.map(|head| unsafe { &*head.as_ptr() })
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        self.head.map(|head| unsafe { &mut *head.as_ptr() })
    }

    pub fn tail(&self) -> Option<&T> {
        self.tail.map(|tail| unsafe { &*tail.as_ptr() })
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        self.tail.map(|tail| unsafe { &mut *tail.as_ptr() })
    }

    pub fn push_head(&self, node: Unique<T>) {
        unimplemented!("");
    }

    pub fn length(&self) -> usize {

        self.length
    }
    
    // pub fn push(&mut self, new_node: NonNull<T>, is_next: bool) {
    //     if let Some(mut node) = self.node {
    //         unsafe {
    //             if is_next {
    //                 node.as_mut().insert_next(new_node);
    //             } else {
    //                 node.as_mut().insert_prev(new_node);
    //             }
    //         }
    //     } else {
    //         self.node = Some(new_node);
    //     }
    //
    //     self.length += 1;
    // }
    //
    // pub fn push_head(&mut self, new_node: NonNull<T>) {
    //     self.push(new_node, true);
    // }
    //
    // pub fn push_tail(&mut self, new_node: NonNull<T>) {
    //     self.push(new_node, false);
    // }
    //
    // fn pop(&mut self, is_next: bool) -> Option<NonNull<T>> {
    //     self.node.map(|mut node| {
    //         if unsafe { node.as_ref().is_alone() } {
    //             self.node = None;
    //         } else {
    //             let node = unsafe { node.as_mut() };
    //             self.node = Some(
    //                 match is_next {
    //                     true => node.next_mut(),
    //                     false => node.prev_mut(),
    //                 }.as_shared(),
    //             );
    //             node.detach();
    //         }
    //
    //         self.length -= 1;
    //         if self.length == 0 {
    //             self.node = None;
    //         }
    //
    //         node
    //     })
    // }
    //
    // pub fn pop_head(&mut self) -> Option<NonNull<T>> {
    //     self.pop(true)
    // }
    //
    // pub fn pop_tail(&mut self) -> Option<NonNull<T>> {
    //     self.pop(false)
    // }
    //
    // fn has_node(&self, target_node: NonNull<T>) -> bool {
    //     match self.node {
    //         None => false,
    //         Some(node) if self.length == 1 => {
    //             ptr::eq(node.as_ptr(), target_node.as_ptr())
    //         },
    //         Some(mut node) => {
    //             let node = unsafe { node.as_mut() };
    //             let tail = node.prev_mut().as_shared();
    //             let mut current = node.next_mut().prev_mut().as_shared();
    //
    //             while ptr::eq(current.as_ptr(), tail.as_ptr()) == false {
    //                 if ptr::eq(current.as_ptr(), target_node.as_ptr()) {
    //                     return true;
    //                 }
    //             }
    //
    //             false
    //         }
    //     }
    // }
    //
    // pub fn detach_node(&mut self, mut node: NonNull<T>) {
    //     debug_assert!(self.has_node(node), "this node does not belong this list");
    //
    //     unsafe { node.as_mut().detach() };
    //     self.length -= 1;
    //
    //     if self.length == 0 {
    //         self.node = None;
    //     }
    // }
}


#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use self::std::heap::{Alloc, Layout, System};
    use self::std::mem;

    #[derive(Debug)]
    struct Object {
        next: Option<NonNull<Object>>,
        prev: Option<NonNull<Object>>,
        order: usize,
        hoge: usize,
        huga: usize,
    }

    // TODO: use macro or custom derive.
    impl Node<Object> for Object {
        fn set_next(&mut self, ptr: NonNull<Self>) {
            self.next = Some(ptr);
        }

        fn set_prev(&mut self, ptr: NonNull<Self>) {
            self.prev = Some(ptr);
        }

        fn next(&self) -> Option<NonNull<Self>> {
            self.next
        }

        fn prev(&self) -> Option<NonNull<Self>> {
            self.prev
        }
    }

    fn allocate_nodes<T>(count: usize) -> *mut T {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();

        ptr.as_ptr() as *mut _
    }

    fn uniqued<T>(nodes: *mut T, index: usize) -> Unique<T> {
        unsafe { Unique::new_unchecked(nodes.offset(index as isize)) }
    }

    #[test]
    fn test_push_head_ones() {
        let mut list1 = LinkedList::<Object>::new();

        assert_eq!(false, list1.head().is_some());

        const SIZE: usize = 8;
        let nodes = allocate_nodes::<Object>(SIZE);

        list1.push_head(uniqued(nodes, 0));

        assert_eq!(true, list1.head().is_some());
        if let Some(head) = list1.head_mut() {
            head.hoge = 10;
            assert_eq!(10, unsafe {(*nodes.offset(0)).hoge});
        } else {
            panic!("error");
        }
    }
}
