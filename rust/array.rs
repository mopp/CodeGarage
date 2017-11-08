#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Frame(usize);

trait FrameAllocator<'a> {
    fn alloc(&'a self, usize) -> &'a [Frame];
    fn free(&self, &[Frame]);
}

struct Allocator([Frame; 5]);

impl<'a> FrameAllocator<'a> for Allocator {
    fn alloc(&'a self, n: usize) -> &'a [Frame] {
        &self.0[0..n]
    }

    fn free(&self, _: &[Frame]) {
        unimplemented!("");
    }
}

fn main() {
    let a = Allocator([
                      Frame(1),
                      Frame(2),
                      Frame(3),
                      Frame(4),
                      Frame(5),
    ]);

    let f = a.alloc(2);
    println!("{:?}", f);

    let f = a.alloc(3);
    println!("{:?}", f);
}
