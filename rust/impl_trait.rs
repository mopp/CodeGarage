#![feature(conservative_impl_trait)]


fn foo(n: u32) -> impl Iterator<Item=u32> {
    (0..n).map(|x| x * 100)
}


fn main() {
    for x in foo(10) {
        println!("{}", x);
    }
}
