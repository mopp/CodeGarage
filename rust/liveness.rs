// https://doc.rust-lang.org/stable/nomicon/references.html#liveness
fn main() {
    let x = &mut (1, 2);
    println!("{:?}", x);

    {
        // reborrow x to a subfield
        let y = &mut x.0;
        // y is now live, but x isn't
        *y = 3;

        // error: cannot assign to `x.0` because it is borrowed [E0506]
        // x.0 = 999;
    }

    println!("{:?}", x);

    // y goes out of scope, so x is live again
    *x = (5, 7);
    println!("{:?}", x);


    let x = &mut (10, 20);
    println!("\n{:?}", x);
    {
        // reborrow x to two disjoint subfields
        // xの子孫である、yとz同士に直接的関連はないためmutableにreferenceをとれる.
        let y = &mut x.0;
        let z = &mut x.1;

        // y and z are now live, but x isn't
        *y = 30;
        *z = 40;
    }
    println!("{:?}", x);

    // y and z go out of scope, so x is live again
    *x = (50, 70);
    println!("{:?}", x);


    let x = &mut 100;
    let y = &200;

    println!("{:?}", x);
    let tmp = *x;
    *x = *y;
    *y = tmp;
    println!("{:?}", x);
}
