// http://rustbyexample.com/custom_types/enum/testcase_linked_list.html
#![feature(box_syntax, box_patterns)]
use std::ops::Add;
use List::*;


#[derive(Debug, PartialEq, Clone)]
enum List<T> {
    Node(T, Box<List<T>>),
    Nil,
}


macro_rules! reverse {
    ($macro:ident, $( $v:tt ),* ) => {
        reverse!($macro, [$( $v )*] [])
    };

    ($macro:ident, [$head:tt $( $rest:tt )*] [$( $reversed:tt )*]) => {
        reverse!($macro, [$( $rest )*] [$head $( $reversed )*])
    };

    ($macro:ident, [] [$( $reversed:tt )*]) => {
        $macro!($( $reversed ),*)
    };
}

macro_rules! list_sub {
    ($( $value:expr ),*) => {
        {
            List::new()$(.push_head($value))*
        }
    };
}

macro_rules! list {
    ($( $value:expr ),*) => {
        reverse!(list_sub, $($value),*)
    };
}


impl<T> List<T> {
    fn new() -> List<T>
    {
        List::Nil
    }

    fn with_value(e: T) -> List<T>
    {
        Node(e, Box::new(Nil))
    }

    fn push_head(self, e: T) -> List<T>
    {
        Node(e, Box::new(self))
    }

    fn push_back(self, e: T) -> List<T>
    {
        match self {
            Nil           => List::with_value(e),
            Node(v, tail) => Node(v, Box::new(tail.push_back(e))),
        }
    }

    fn pop_head(self) -> (Option<T>, List<T>)
    {
        match self {
            Nil           => (None, Nil),
            Node(v, box tail) => (Some(v), tail),
        }
    }

    fn length(&self) -> usize
    {
        match *self {
            Node(_, ref tail) => 1 + tail.length(),
            Nil               => 0
        }
    }

    fn last(self) -> T
    {
        match self {
            Nil => panic!("List::Nil cannot have any value."),
            Node(v, box Nil) => v,
            Node(_, tail) => tail.last(),
        }
    }

    fn nth(self, idx: usize) -> Option<T>
    {
        fn f<T>(list: List<T>, idx: usize, count: usize) -> Option<T>
        {
            if let Node(v, box tail) = list {
                if count == idx {
                    return Some(v);
                }
                f(tail, idx, count + 1)
            } else {
                None
            }
        }

        f(self, idx, 0)
    }
}


impl<T> Add for List<T> {
    type Output = List<T>;

    fn add(self, right: List<T>) -> List<T>
    {
        match self {
            Nil           => right,
            Node(v, tail) => Node(v, Box::new(tail.add(right))),
        }
    }
}

fn main()
{
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new()
    {
        assert_eq!(List::<usize>::new(), Nil)
    }

    #[test]
    fn test_push_head()
    {
        let n = List::<usize>::new();
        let n = n.push_head(100);
        assert_eq!(n, Node(100, Box::new(Nil)));
        let n = n.push_head(200);
        assert_eq!(n, Node(200, Box::new(Node(100, Box::new(Nil)))));
    }

    #[test]
    fn test_push_back()
    {
        let n = List::<usize>::new();
        let n = n.push_back(100);
        assert_eq!(n, Node(100, Box::new(Nil)));
        let n = n.push_back(200);
        assert_eq!(n, Node(100, Box::new(Node(200, Box::new(Nil)))));
    }

    #[test]
    fn test_pop_head()
    {
        let n = list![1, 2, 3, 4];

        let (v, n) = n.pop_head();
        assert_eq!(v, Some(1));
        assert_eq!(n, list![2, 3, 4]);

        let (v, n) = n.pop_head();
        assert_eq!(v, Some(2));
        assert_eq!(n, list![3, 4]);

        let (v, n) = n.pop_head();
        assert_eq!(v, Some(3));
        assert_eq!(n, list![4]);

        let (v, n) = n.pop_head();
        assert_eq!(v, Some(4));
        assert_eq!(n, Nil);

        let (v, n) = n.pop_head();
        assert_eq!(v, None);
        assert_eq!(n, Nil);
    }

    #[test]
    fn test_length()
    {
        let n = List::<usize>::new();
        let n = n.push_head(100);
        assert_eq!(n.length(), 1);

        let n = n.push_head(200);
        assert_eq!(n.length(), 2);

        let n = list![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(n.length(), 8);
    }

    #[test]
    fn test_last()
    {
        assert_eq!(list![1, 2, 3, 4, 5, 6, 7, 8].last(), 8);
        assert_eq!(list![1].last(), 1);
    }

    #[test]
    fn test_nth()
    {
        assert_eq!(list![1, 2, 3, 4, 5, 6, 7, 8].nth(0), Some(1));
        assert_eq!(list![1, 2, 3, 4, 5, 6, 7, 8].nth(7), Some(8));
        assert_eq!(list![1].nth(100), None);
    }

    #[test]
    fn test_add()
    {
        let n1 = list![4, 5];
        assert_eq!(Nil + n1, list![4, 5]);

        let n1 = list![1, 2, 3];
        let n2 = list![4, 5];
        assert_eq!(n1.add(n2), list![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_list_macro()
    {
        let n1 = List::<usize>::new().push_back(100).push_back(200).push_back(300).push_back(400);
        let n2 = list![100, 200, 300, 400];
        assert_eq!(n1, n2);

        let n1 = List::<usize>::new();
        let n2 = list![];
        assert_eq!(n1, n2);
    }
}
