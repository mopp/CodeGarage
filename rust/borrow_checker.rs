// http://stackoverflow.com/questions/24847533/why-does-rust-borrow-checker-reject-this-code?rq=1

#[derive(Debug)]
struct A(usize);

#[derive(Debug)]
struct B<'a> {
    a: &'a A,
    cnt: usize,
}

impl<'a> B<'a> {
    fn refer(&mut self) -> &'a A {
        self.cnt += 1;
        self.a
    }
}

fn main() {
    let mut b = B { a: &mut A(10), cnt:0 };

    let f = b.refer();
    println!("{:?}", f);

    let g = b.refer();
    println!("{:?}", g);
}
