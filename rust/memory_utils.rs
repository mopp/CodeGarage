// https://doc.rust-lang.org/std/mem/index.html
use std::{mem, ptr};


fn main()
{
    uninitialized_memory();
    align_of_memory();
    dispose_value();
}


fn uninitialized_memory()
{
    let mut data: [usize; 1000];

    unsafe {
        data = mem::uninitialized();

        for elem in &mut data[..] {
            ptr::write(elem, 100usize);
        }
    }

    for elem in &mut data[..] {
        assert_eq!(*elem, 100);
    }
}


fn align_of_memory()
{
    assert_eq!(4, mem::align_of::<i32>());
    assert_eq!(8, mem::align_of::<i64>());

    struct ObjA {
        i: i32
    }

    struct ObjB {
        i: i32,
        j: i32
    }

    struct ObjC {
        i: u8
    }

    struct ObjD {
        i: u8,
        j: u8
    }

    struct ObjE {
        i: u8,
        j: u8,
        k: i32,
    }

    struct ObjF {
        i: u8,
        j: u8,
        k: u64,
    }

    assert_eq!(4, mem::align_of::<ObjA>());
    assert_eq!(4, mem::align_of::<ObjB>());
    assert_eq!(1, mem::align_of::<ObjC>());
    assert_eq!(1, mem::align_of::<ObjD>());
    assert_eq!(4, mem::align_of::<ObjE>());
    assert_eq!(8, mem::align_of::<ObjF>());
}


fn dispose_value()
{
    let v = vec![1, 2, 3];
    drop(v);

    // Error: value used here after move.
    // println!("{:?}", v);

    let mut v = vec![1, 2, 3];
    let x = &v[0];

    // explicitly drop the reference, but the borrow still exists
    // borrowの先は解放されない
    drop(x);

    v.push(4); // error: cannot borrow `v` as mutable because it is also borrowed as immutable
}
