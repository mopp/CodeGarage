// https://doc.rust-lang.org/nightly/nomicon/repr-rust.html

use std::mem;

struct A {
    a: u8,
    b: u32,
    c: u16,
}

struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U,
}

type Foo1 = Foo<u16, u32>;  // No padding.
type Foo2 = Foo<u32, u16>;  // Waste memory because of padding.

struct NoSize; // No fields = no size

// All fields have no size = no size
struct Baz {
    foo: NoSize,
    qux: (),      // empty tuple has no size
    baz: [u8; 0], // empty array has no size
}


fn main()
{
    println!("The size of A = {} byte.", mem::size_of::<A>());
    assert_eq!(12, mem::size_of::<A>());

    println!("The size of Foo1 = {} byte.", mem::size_of::<Foo1>());
    assert_eq!(8, mem::size_of::<Foo1>());
    println!("The size of Foo2 = {} byte.", mem::size_of::<Foo2>());
    assert_eq!(12, mem::size_of::<Foo2>());

    println!("The size of Baz = {} byte.", mem::size_of::<Baz>());
    assert_eq!(0, mem::size_of::<Baz>());
}
