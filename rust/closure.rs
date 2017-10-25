// http://keens.github.io/blog/2016/10/10/rustnokuro_ja3tanewotsukutterikaisuru/
#![feature(unboxed_closures)]
#![feature(fn_traits)]


struct Closure {
    i: isize,
}


impl Closure {
    fn new(i: isize) -> Closure {
        Closure {
            i: i
        }
    }
}


impl FnOnce<(isize,)> for Closure {
    type Output = isize;

    extern "rust-call" fn call_once(self, (arg, ): (isize,)) -> Self::Output {
        self.i + arg
    }
}


impl FnMut<(isize,)> for Closure {
    extern "rust-call" fn call_mut(&mut self, (arg, ): (isize,)) -> Self::Output {
        self.i + arg
    }
}


impl Fn<(isize,)> for Closure {
    extern "rust-call" fn call(&self, (arg,): (isize, )) -> Self::Output {
        self.i + arg
    }
}


fn main() {
    // FnOnce
    {
        let x = 1;
        for i in 0..10 {
            let cls = Closure::new(i);
            println!("{}", cls(x));
        }
    }

    // FnMut
    {
        let x = 1;
        let cls = Closure::new(x);
        for i in 0..10 {
            println!("{}", cls(i));
        }
    }
}
