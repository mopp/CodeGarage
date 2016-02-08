// Invalid

#![feature(shared)]
// #![no_std]

extern crate core;
use core::ptr::Shared;
use core::cmp::PartialEq;


type Link<T> = Option<Shared<WList<T>>>;


struct WList<T>  {
    front: Link<T>,
    back: Link<T>,
    content: Shared<T>
}


// Static methods.
impl<T> WList<T>  {
    fn make_link(node: &mut WList<T>) -> Link<T>
    {
        Some(unsafe {Shared::new(node)})
    }


    fn link<'a>(l: Link<T>) -> Option<&'a WList<T>>
    {
        match l {
            None => None,
            Some(c) => Some(unsafe {&**c}),
        }
    }


    fn link_mut<'a>(l: Link<T>) -> Option<&'a mut WList<T>>
    {
        match l {
            None => None,
            Some(c) => Some(unsafe {&mut **c}),
        }
    }


    fn link_each_other(node1: &mut WList<T>, node2: &mut WList<T>)
    {
        let node1_opt = WList::make_link(node1);
        let node2_opt = WList::make_link(node2);

        node1.front = node2_opt;
        node1.back  = node2_opt;
        node2.front = node1_opt;
        node2.back  = node1_opt;
    }


    fn push_guard(pushed: &WList<T>, new: &WList<T>) {
        if new.is_empty() == false {
            panic!("New node should NOT be linked to other nodes.");
        }

        if new == pushed {
            panic!("We cannot push self into self.");
        }
    }
}


impl<T> WList<T> {
    fn new(content: &mut T) -> WList<T>
    {
        WList {
            front: None,
            back: None,
            content: unsafe {Shared::new(content)},
        }
    }


    fn iter(&self) -> Iter<T>
    {
        Iter {
            begin_node: self,
            current_node: self,
            is_first: true,
        }
    }


    fn iter_mut(&self) -> IterMut<T>
    {
        IterMut {
            begin_node: self,
            current_node: self,
            is_first: true,
        }
    }


    fn is_empty(&self) -> bool
    {
        match (self.front, self.back) {
            (None, None) => true,
            _            => false,
        }
    }


    fn front(&self) -> Option<&WList<T>>
    {
        WList::link(self.front)
    }


    fn front_mut(&self) -> Option<&mut WList<T>>
    {
        WList::link_mut(self.front)
    }


    fn back(&self) -> Option<&WList<T>>
    {
        WList::link(self.back)
    }


    fn back_mut(&self) -> Option<&mut WList<T>>
    {
        WList::link_mut(self.back)
    }


    fn push_front(&mut self, mut new: WList<T>)
    {
        WList::push_guard(self, &new);

        if self.is_empty() == true {
            // If self is not linked by any other nodes.
            WList::link_each_other(self, &mut new);
            return;
        }

        let self_opt = WList::make_link(self);
        new.front = self.front;
        new.back = self_opt;

        let new_opt = WList::make_link(&mut new);
        self.front_mut().unwrap().back = new_opt;
        self.front = new_opt;
    }


    fn pop_front(&mut self) -> Option<Shared<WList<T>>>
    {
        match self.front {
            None => None,
            Some(shared_front) => {
                let front  = unsafe { &**shared_front };
                self.front = front.front;
                let tmp    = unsafe { &mut **self.front.unwrap()};
                tmp.back   = WList::make_link(self);

                Some(shared_front)
            }
        }
    }


    fn push_back(&mut self, mut new: WList<T>)
    {
        WList::push_guard(self, &new);

        if self.is_empty() == true {
            // If self is not linked by any other nodes.
            WList::link_each_other(self, &mut new);
            return;
        }

        let self_opt = WList::make_link(self);
        new.front = self_opt;
        new.back  = self.back;

        let new_opt = WList::make_link(&mut new);
        self.back_mut().unwrap().front = new_opt;
        self.back = new_opt;
    }


    fn pop_back(&mut self) -> Option<&mut T>
    {
        match self.back {
            None => None,
            Some(shared_back) => {
                let back  = unsafe { &**shared_back };
                self.back = back.back;
                let tmp   = unsafe { &mut **self.back.unwrap()};
                tmp.front = WList::make_link(self);

                Some(back.borrow_mut())
            }
        }
    }


    fn borrow(&self) -> &T
    {
        unsafe {
            &**self.content
        }
    }


    fn borrow_mut(&self) -> &mut T
    {
        unsafe{&mut **self.content}
    }
}


impl<T> PartialEq for WList<T> {
    fn eq(&self, other: &WList<T>) -> bool
    {
        let addr_self  = (self as *const _) as usize;
        let addr_other = (other as *const _) as usize;
        addr_self == addr_other
    }
}


macro_rules! def_iter_struct {
    ($i:ident) => {
        struct $i<'a, T: 'a> {
            begin_node: &'a WList<T>,
            current_node: &'a WList<T>,
            is_first: bool,
        }
    };
}


def_iter_struct!(Iter);


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let old_current_node = self.current_node;
        let value            = old_current_node.borrow();
        self.current_node    = old_current_node.front().unwrap();

        if self.is_first {
            self.is_first = false;
            return Some(value);
        }

        if self.begin_node == old_current_node {
            return None;
        }

        Some(value)
    }
}


def_iter_struct!(IterMut);


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let old_current_node = self.current_node;
        let value            = old_current_node.borrow_mut();
        self.current_node    = old_current_node.front().unwrap();

        if self.is_first {
            self.is_first = false;
            return Some(value);
        }

        // println!("{:x} vs {:x}", (self.begin_node  as *const _) as usize, (old_current_node  as *const _) as usize);
        if self.begin_node == old_current_node {
            return None;
        }

        Some(value)
    }
}


#[cfg(test)]
mod test {
    use super::WList;

    #[test]
    fn test_push_front()
    {
        let mut n1 = WList::<usize>::new(&mut 0);
        let n2     = WList::<usize>::new(&mut 1);

        n1.push_front(n2);

        assert_eq!(0, *n1.borrow());
        assert_eq!(1, *n1.front().unwrap().borrow());
        assert_eq!(1, *n1.back().unwrap().borrow());
        {
            let n2 = n1.back().unwrap();
            assert_eq!(0, *n2.front().unwrap().borrow());
            assert_eq!(0, *n2.back().unwrap().borrow());
        }

        *n1.borrow_mut() = 7;
        {
            let n2 = n1.front().unwrap();
            assert_eq!(7, *n2.front().unwrap().borrow());
            assert_eq!(7, *n2.back().unwrap().borrow());
        }

        let n3 = WList::<usize>::new(&mut 2);
        n1.push_front(n3);
        {
            let n2 = n1.back().unwrap();
            assert_eq!(1, *n2.borrow());
            assert_eq!(2, *n2.back().unwrap().borrow());
            assert_eq!(7, *n2.back().unwrap().back().unwrap().borrow());
        }

        {
            let n3 = n1.front().unwrap();
            assert_eq!(2, *n3.borrow());
            assert_eq!(1, *n3.front().unwrap().borrow());
            assert_eq!(7, *n3.front().unwrap().front().unwrap().borrow());
        }

        let n4 = WList::<usize>::new(&mut 3);
        n1.push_front(n4);
        {
            let n4 = n1.front().unwrap();
            assert_eq!(3, *n4.borrow());
            assert_eq!(2, *n4.front().unwrap().borrow());
            assert_eq!(1, *n4.front().unwrap().front().unwrap().borrow());
            assert_eq!(7, *n4.back().unwrap().borrow());
        }
    }


    #[test]
    fn test_push_back()
    {
        let mut n1 = WList::<usize>::new(&mut 0);
        let n2 = WList::<usize>::new(&mut 1);

        n1.push_back(n2);

        assert_eq!(1, *n1.front().unwrap().borrow());
        assert_eq!(1, *n1.back().unwrap().borrow());
        {
            let n2 = n1.back().unwrap();
            assert_eq!(0, *n2.front().unwrap().borrow());
            assert_eq!(0, *n2.back().unwrap().borrow());
        }

        *n1.borrow_mut() = 7;
        {
            let n2 = n1.front().unwrap();
            assert_eq!(7, *n2.front().unwrap().borrow());
            assert_eq!(7, *n2.back().unwrap().borrow());
        }

        let n3 = WList::<usize>::new(&mut 2);
        n1.push_back(n3);
        {
            let n3 = n1.back().unwrap();
            assert_eq!(2, *n3.borrow());
            assert_eq!(1, *n3.back().unwrap().borrow());
            assert_eq!(7, *n3.back().unwrap().back().unwrap().borrow());
        }

        {
            let n2 = n1.front().unwrap();
            assert_eq!(1, *n2.borrow());
            assert_eq!(2, *n2.front().unwrap().borrow());
            assert_eq!(7, *n2.front().unwrap().front().unwrap().borrow());
        }

        let n4 = WList::<usize>::new(&mut 3);
        n1.push_back(n4);
        {
            let n4 = n1.back().unwrap();
            assert_eq!(3, *n4.borrow());
            assert_eq!(7, *n4.front().unwrap().borrow());
            assert_eq!(1, *n4.front().unwrap().front().unwrap().borrow());
            assert_eq!(2, *n4.back().unwrap().borrow());
        }
    }

    macro_rules! def_iter_struct {
        () => {
        };
    }

    #[test]
    fn test_itr()
    {
        let mut n1 = WList::new(&mut 0);
        let n2 = WList::new(&mut 1);
        let n3 = WList::new(&mut 2);
        let n4 = WList::new(&mut 3);

        n1.push_front(n2);
        n1.front_mut().unwrap().push_front(n3);
        n1.front_mut().unwrap().front_mut().unwrap().push_front(n4);

        let mut itr = n1.iter();
        assert_eq!(0, *itr.next().unwrap());
        assert_eq!(1, *itr.next().unwrap());
        assert_eq!(2, *itr.next().unwrap());
        assert_eq!(3, *itr.next().unwrap());
        assert_eq!(None, itr.next());

        let mut cnt = 0;
        let ans = [0, 1, 2, 3];
        for i in n1.iter() {
            assert_eq!(ans[cnt], *i);
            cnt += 1;
        }
    }


    #[test]
    fn test_itr_mut()
    {
        let n1 =
            {
                let mut n1 = WList::new(&mut 0);
                let n2 = WList::new(&mut 1);
                let n3 = WList::new(&mut 2);
                let n4 = WList::new(&mut 3);

                n1.push_front(n2);
                n1.front_mut().unwrap().push_front(n3);
                n1.front_mut().unwrap().front_mut().unwrap().push_front(n4);

                println!("0x{:x}", (&n1  as *const _) as usize);
                n1
            };
        println!("0x{:x}", (&n1  as *const _) as usize);

        let mut cnt = 0;
        let ans = [0, 1, 2, 3];
        // for i in n1.iter_mut() {
            // println!("{:?}", i);
            // assert_eq!(ans[cnt], *i);
            // cnt += 1;

            // *i += 999;
        // }

        cnt = 0;
        for i in n1.iter() {
            assert_eq!(ans[cnt] + 999, *i);
            cnt += 1;
        }
    }


    #[test]
    fn test_pop_front()
    {
        let mut n1 = WList::new(&mut 0);
        let n2 = WList::new(&mut 1);
        let n3 = WList::new(&mut 2);
        let n4 = WList::new(&mut 3);

        n1.push_front(n2);
        n1.front_mut().unwrap().push_front(n3);
        n1.front_mut().unwrap().front_mut().unwrap().push_front(n4);

        let mut cnt = 0;
        let ans = [0, 1, 2, 3];
        for i in n1.iter() {
            assert_eq!(ans[cnt], *i);
            cnt += 1;
        }

        let content = {*unsafe {&**(n1.pop_front().unwrap())}.borrow()};
        assert_eq!(1, content);

        let mut cnt = 0;
        let ans = [0, 2, 3];
        for i in n1.iter() {
            assert_eq!(ans[cnt], *i);
            cnt += 1;
        }
    }
}
