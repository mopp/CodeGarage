struct Obj(usize);

fn hoge(v: Obj)
{
    let a = 100;
    println!("0x{:p}", &v);
    println!("0x{:p}", &a);
}

fn main()
{
    let v = Obj(200);
    hoge(v);
}
