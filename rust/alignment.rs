use std::mem::size_of;


struct A {
    a: u8,   // 1
    b: u32,  // 4
    c: u16,  // 2
}

// struct Bとstruct Cで
// フィールドaとbのメモリ上の順番が同じである保証はない
struct B {
    a: i32,
    b: u64,
}

struct C {
    a: i32,
    b: u64,
}

struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U,
}

enum Hoge {
    A(u32),
    B(u64),
    C(u8),
}

struct Piyo;

fn main()
{
    assert_eq!(8, size_of::<A>());
    assert_eq!(16, size_of::<B>());
    assert_eq!(16, size_of::<C>());

    // u16, u16, u32で8バイトアライン
    assert_eq!(8, size_of::<Foo<u16, u32>>());
    // u16, u32, u32だが、フィールドの順序を最適化のために入れ替えているので8バイトアライン
    assert_eq!(8, size_of::<Foo<u32, u16>>());

    // enumのアラインメントを考えるのはもっと面倒
    assert_eq!(16, size_of::<Hoge>());

    assert_eq!(0, size_of::<Piyo>());
}
