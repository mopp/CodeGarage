// [Higher-Rank Trait Bounds -](https://doc.rust-lang.org/nomicon/hrtb.html)
// [rfcs/0387-higher-ranked-trait-bounds.md at master · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md)

mod sample1 {
    struct ObjextX(usize, isize);

    struct Closure<F> {
        data: ObjextX,
        func: F,
    }

    impl<F> Closure<F> where for<'a> F: Fn(&'a ObjextX) -> &'a usize {
        fn call<'a>(&'a self) -> &'a usize {
            (self.func)(&self.data)
        }
    }

    fn do_it<'b>(data: &'b ObjextX) -> &'b usize {
        &data.0
    }

    pub fn main() {
        let clo = Closure {
            data: ObjextX(0, 1),
            func: do_it
        };

        println!("{}", clo.call());
        assert_eq!(&0, clo.call());
    }
}


fn main() {
    sample1::main();
}
