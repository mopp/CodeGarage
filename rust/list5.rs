#![feature(shared)]
#![feature(unique)]
#![feature(allocator_api)]

use std::default::Default;
use std::ptr::Shared;
use std::ptr::Unique;
use std::mem;


struct LinkedList<T: Default> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
}


struct Node<T: Default> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    element: T,
}

impl<T: Default> Default for Node<T> {
    fn default() -> Node<T>
    {
        Node {
            next: None,
            prev: None,
            element: Default::default(),
        }
    }
}


impl<T: Default> LinkedList<T> {
    fn new() -> LinkedList<T>
    {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn len(&self) -> usize
    {
        let mut node =
            if let Some(ref head) = self.head {
                unsafe { head.as_ref() }
            } else  {
                return 0;
            };

        let mut cnt = 1;
        loop {
            match node.next {
                None => break,
                Some(ref next) => {
                    node = unsafe {next.as_ref()};
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn front(&self) -> Option<&T>
    {
        unsafe {
            self.head.as_ref().map(|node| &node.as_ref().element)
        }
    }

    fn front_mut(&mut self) -> Option<&mut T>
    {
        unsafe {
            self.head.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    fn back(&self) -> Option<&T>
    {
        unsafe {
            self.tail.as_ref().map(|node| &node.as_ref().element)
        }
    }

    fn back_mut(&mut self) -> Option<&mut T>
    {
        unsafe {
            self.tail.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    fn push_front(&mut self, new_node: *mut Node<T>)
    {
        let mut new_shared_node = unsafe { Shared::new_unchecked(new_node) };

        {
            let n = unsafe { new_shared_node.as_mut() };
            n.next = self.head;
            n.prev = None;
        }

        let new_shared_node = Some(new_shared_node);
        match self.head {
            None           => self.tail = new_shared_node,
            Some(mut head) => unsafe { head.as_mut().prev = new_shared_node },
        }

        self.head = new_shared_node;
    }

    fn push_back(&mut self, new_node: *mut Node<T>)
    {
        let mut new_shared_node = unsafe { Shared::new_unchecked(new_node) };

        {
            let n = unsafe { new_shared_node.as_mut() };
            n.next = None;
            n.prev = self.tail;
        }

        let new_shared_node = Some(new_shared_node);
        match self.tail {
            None  => self.head = new_shared_node,
            Some(mut tail) => unsafe {tail.as_mut().next = new_shared_node},
        }

        self.tail = new_shared_node;
    }

    fn pop_front(&mut self) -> Option<*mut Node<T>>
    {
        match self.head {
            None       => None,
            Some(head) => {
                self.head = unsafe { head.as_ref().next };

                match self.head {
                    None               => self.tail = None,
                    Some(mut new_head) => unsafe { new_head.as_mut().prev = None },
                }

                Some(head.as_ptr())
            }
        }
    }

    fn pop_back(&mut self) -> Option<*mut Node<T>>
    {
        match self.tail {
            None       => None,
            Some(tail) => {
                self.tail = unsafe { tail.as_ref().prev };

                match self.tail {
                    None               => self.head = None,
                    Some(mut new_tail) => unsafe { new_tail.as_mut().next = None },
                }

                Some(tail.as_ptr())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::heap::{Alloc, System, Layout};
    use std::slice;

    fn allocate_unique_objs<'a, T>(count: usize) -> &'a mut [T] where T: Default
    {
        let type_size = mem::size_of::<T>();
        let align     = mem::align_of::<T>();
        let layout    = Layout::from_size_align(count * type_size, align).unwrap();
        let ptr       = unsafe { System.alloc(layout) }.unwrap();
        unsafe { slice::from_raw_parts_mut(ptr as *mut T, count) }
    }

    #[test]
    fn test_push_front()
    {
        let objs = allocate_unique_objs::<Node<usize>>(1024);

        let mut list = LinkedList::new();
        list.push_front(&mut objs[0] as *mut _);
        assert_eq!(list.len(), 1);
        assert_eq!(list.back(), Some(&0usize));
        assert_eq!(list.front(), Some(&0usize));
    }

    #[test]
    fn test_push_back()
    {
        let objs = allocate_unique_objs::<Node<usize>>(1024);

        let mut list = LinkedList::new();
        list.push_back(&mut objs[1] as *mut _);
        assert_eq!(list.len(), 1);
        assert_eq!(list.back(), Some(&0usize));
        assert_eq!(list.front(), Some(&0usize));
    }

    #[test]
    fn test_pop_front()
    {
        let mut objs = allocate_unique_objs::<Node<usize>>(128);

        let mut list = LinkedList::new();
        for (i, o) in objs.iter_mut().enumerate() {
            o.element = i;

            list.push_front(o);
        }

        assert_eq!(list.len(), objs.len());
        assert_eq!(list.back(), Some(&0usize));
        assert_eq!(list.front(), Some(&(objs.len() - 1)));

        for i in (0..objs.len()).rev() {
            let n = list.pop_front();
            assert_eq!(i, unsafe {(*n.unwrap()).element});
        }
    }

}
