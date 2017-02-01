fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let pair: Vec<(&usize, &usize)> = a1.iter().zip(a2.iter()).collect();

    println!("{:?}", pair);
    assert_eq!(Some(pair[0]), Some((&1, &4)));
    assert_eq!(Some(pair[1]), Some((&2, &5)));
    assert_eq!(Some(pair[2]), Some((&3, &6)));

    let s1 = ["Ann", "Bob", "Someone"];
    let s2 = ["ann", "bob", "someone"];

    let pair: Vec<(&&str, &&str)> = s1.iter().zip(s2.iter()).collect();
    println!("{:?}", pair);
}
