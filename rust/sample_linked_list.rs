fn main()
{
    use std::collections::LinkedList;

    let mut list1 = LinkedList::new();
    list1.push_back('a');
    list1.push_back('b');
    list1.push_back('c');

    {
        let mut iter = list1.into_iter().filter(|x| x == &'b');
        println!("{:?}", iter.next());
    }

    println!("{:?}", list1);
}
