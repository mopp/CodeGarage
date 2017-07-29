#[derive(Debug)]
struct A(usize, usize);

fn mover(a: A) -> A
{
    let A(x, y) = a;
    A(x + 100, y + 100)
}

fn main()
{
    let a = A(1, 1);
    println!("{:?}", a);

    let b = mover(a);
    // error[E0382]: use of moved value: `a`
    // println!("{:?}", a);
    println!("{:?}", b);
}
