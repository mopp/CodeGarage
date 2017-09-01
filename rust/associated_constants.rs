// http://0x90.hatenablog.jp/entry/2017/08/26/002324
trait HasConst {
    type T;
    const CONSTANT_VALUE: Self::T;

    fn constant_value(&self) -> Self::T
    {
        Self::CONSTANT_VALUE
    }
}

struct A;
struct B;

impl HasConst for A {
    type T = f64;
    const CONSTANT_VALUE: f64 = 0f64;
}

impl HasConst for B {
    type T = i64;
    const CONSTANT_VALUE: i64 = 1i64;
}

fn main()
{
    println!("A {:?}", A.constant_value());
    println!("B {:?}", B.constant_value());
}
