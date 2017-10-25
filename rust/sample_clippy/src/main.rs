// Default build is stable.
// clippy is enable when build on nightly and opted.
// ```sh
// rustup default stable
// cargo build
// cargo +nightly build --features clippy
// ```

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// These cause box_vec warning.
// https://rust-lang-nursery.github.io/rust-clippy/v0.0.165/index.html#box_vec
struct Foo;
struct X {
    values: Box<Vec<Foo>>,
}

fn main() {
    println!("{}", None.unwrap_or("unwrap"));
    println!("Hello, world!");
}
