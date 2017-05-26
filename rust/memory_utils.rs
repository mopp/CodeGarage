use std::mem;
use std::ptr;

fn main()
{
    uninitialized_memory();
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
