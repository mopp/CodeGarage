// https://blog.rust-lang.org/2017/07/20/Rust-1.19.html

#[derive(Copy, Clone)]
union MyUnion {
    u0: u32,
    u1: i32,
    f1: f32,
}

#[derive(Copy, Clone)]
struct Hoge {
    n: u32,
}

union NestedUnion {
    x1: MyUnion,
    m: Hoge,
}

fn main()
{
    let mut u = MyUnion { u0: 1 };

    u.u0 = 512;

    unsafe {
        println!("0x{:x}", u.u0);
        println!("u0 = {}", u.u0);
        println!("u1 = {}", u.u1);
        println!("f1 = {}", u.f1);
    }

    let mut u = NestedUnion { x1: MyUnion { u0: 100} };

    u.x1.u0 = 256;
    unsafe {
        println!("0x{:x}", u.m.n);
        println!("x1.u0 = {}", u.x1.u0);
        println!("m.n = {}", u.m.n);
    }
}
