#![feature(unique)]
#![feature(ptr_internals)]
#![cfg_attr(test, feature(allocator_api))]

// use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, Unique};

#[derive(Debug)]
struct Anchor<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Anchor<T> {
    pub fn new() -> Anchor<T> {
        Anchor {
            next: None,
            prev: None,
        }
    }
}

trait Node<T: Node<T>> {
    fn anchor(&self) -> &Anchor<T>;
    fn anchor_mut(&mut self) -> &mut Anchor<T>;
    fn extract(&self) -> &T;
    fn extract_mut(&mut self) -> &mut T;
}

#[derive(Debug)]
struct LinkedList<T: Node<T>> {
    head: Option<NonNull<T>>,
    tail: Option<NonNull<T>>,
    length: usize,
}

impl<T: Node<T>> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
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

    pub fn push_head(&mut self, node: Unique<T>) {
        self.head = Some(NonNull::from(node));
    }

    pub fn pop_head(&mut self) -> Option<Unique<T>> {
        self.head.map(|head| head.into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::heap::{Alloc, System, Layout};
    use std::mem;

    fn allocate_nodes<T>(count: usize) -> *mut T {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr = unsafe { System.alloc(layout) }.unwrap();

        ptr.as_ptr() as *mut _
    }

    #[repr(C)]
    #[derive(Debug)]
    struct Object {
        anchor: Anchor<Object>,
        order: usize,
        hoge: usize,
        huga: usize,
    }

    // TODO: use macro or custom derive.
    impl Node<Object> for Object {
        fn anchor(&self) -> &Anchor<Object> {
            &self.anchor
        }

        fn anchor_mut(&mut self) -> &mut Anchor<Object> {
            &mut self.anchor
        }

        fn extract(&self) -> &Object {
            self
        }

        fn extract_mut(&mut self) -> &mut Object {
            self
        }
    }

    impl Object {
        pub fn new() -> Object {
            Object {
                anchor: Anchor::new(),
                order: 0,
                hoge: 1,
                huga: 2,
            }
        }
    }

    // #[test]
    fn main() {
        let mut list1 = LinkedList::<Object>::new();
        let mut list2 = LinkedList::<Object>::new();

        const SIZE: usize = 8;
        let nodes = allocate_nodes::<Object>(SIZE);

        unsafe {
        println!("{:?}", nodes.offset(0).as_ref());
        list1.push_head(Unique::new_unchecked(nodes.offset(0)));
        list1.push_head(Unique::new_unchecked(nodes.offset(1)));
        }

        let cnt = 2;
        while cnt != 0 {
            if let Some(mut n) = list1.pop_head() {
                // Use note got.
                unsafe {
                    n.as_mut().hoge = 10;
                    n.as_mut().huga = 7;
                    println!("{:?}", n);
                    println!("{:?}", n.as_mut());
                    list2.push_head(n)
                }
            }
            break
        }
    }

    #[test]
    fn push_head() {
        let mut list1 = LinkedList::<Object>::new();

        assert_eq!(false, list1.head().is_some());

        const SIZE: usize = 8;
        let nodes = allocate_nodes::<Object>(SIZE);

        list1.push_head(unsafe { Unique::new_unchecked(nodes.offset(0)) });

        assert_eq!(true, list1.head().is_some());
        if let Some(head) = list1.head_mut() {
            head.hoge = 10;
            assert_eq!(10, unsafe {(*nodes.offset(0)).hoge});
        } else {
            panic!("error");
        }
    }
}
