// https://gist.github.com/14427/af90a21b917d2892eace
// https://gist.github.com/badboy/6954cc6ce1c71b921094

// 1. Functor
// (A => B) => (C<A> => C<B>)
trait Functor<A, B, T> {
    fn map<F>(self, f: F) -> T where F: Fn(A) -> B;
}

// 2. Applicative
// C<A => B> => (C<A> => C<B>)
// trait ApplicativeFunctor<A, B, C, S, T> {
//     fn point(self, a: A) -> S;
//     fn ap<F>(self, f: F) -> T where F: Fn(A) -> B;
// }


// 3. Monad
// (A => C<B>) => (C<A> => C<B>)


// TODO: learn where clause
impl<A, B> Functor<A, B, Option<B>> for Option<A> {
    fn map<F>(self, f: F) -> Option<B> where F: Fn(A) -> B
    {
        match self {
            Some(a) => Some(f(a)),
            None => None,
        }
    }
}


fn main()
{
    let to_string = |x: usize| x.to_string();
    let opt_usize: Option<usize> = Some(100);
    assert_eq!(opt_usize.map(to_string), Some("100".to_string()));
    sub::test();
}


mod sub {
    // http://keens.github.io/blog/2016/02/28/rustnohigherkinded_type_trait/
    // https://gist.github.com/14427/af90a21b917d2892eace

    pub trait HKT<U> {
        type C; // Current type
        type T; // Type with C swapped with U
    }

    pub trait Functor<U>: HKT<U> {
        fn map<F>(&self, f: F) -> Self::T where F: Fn(&Self::C) -> U;
    }

    pub trait Applicative<U>: Functor<U> {
        fn pure_(value: U) -> Self::T where Self: HKT<U, C=U>;
        fn seq<F>(&self, <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where Self: HKT<F>, F: Fn(&<Self as HKT<F>>::C) -> U;
    }

    pub trait Monad<U>: Applicative<U> {
        fn bind<F>(&self, F) -> Self::T where F : FnMut(&Self::C) -> Self::T;

        fn return_(x: U) -> Self::T where Self: HKT<U, C=U> {
            Self::pure_(x)
        }

        fn join<T>(&self) -> T where Self: HKT<U, T=T, C=T>, T: Clone {
            self.bind(|x| x.clone())
        }
    }

    impl<T, U> HKT<U> for Option<T> {
        type C = T;
        type T = Option<U>;
    }

    impl<T, U> Functor<U> for Option<T> {
        fn map<F>(&self, f: F) -> Option<U> where F: Fn(&T) -> U {
            match *self {
                Some(ref value) => Some( f(value) ),
                None => None,
            }
        }
    }

    impl<T, U> Applicative<U> for Option<T> {
        fn pure_(value: U) -> <Self as HKT<U>>::T { Some(value) }

        fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T where F: Fn(&<Self as HKT<F>>::C) -> U {
            match *self {
                Some(ref value) => match fs {
                    Some(f) => Some( f(value) ),
                    None => None,
                },
                None => None,
            }
        }
    }

    impl<T, U> Monad<U> for Option<T> {
        fn bind<F>(&self, mut f: F) -> Option<U> where F : FnMut(&T) -> Option<U> {
            match *self {
                Some(ref value) => f(value),
                None => None,
            }
        }
    }

    pub fn test()
    {
        let to_string = |x: usize| x.to_string();
        let opt_usize: Option<usize> = Some(100);
        assert_eq!(opt_usize.map(to_string), Some("100".to_string()));

        let f1: &Fn(&i32) -> i32 = &|x| x*3;
        let f = Some(f1);
        let n: Option<&Fn(&i32)->i32> = None;
        assert_eq!(Option::pure_(100).seq(f), Some(300));
        assert_eq!(Option::pure_(100).seq(n), None);

        assert_eq!(Option::return_(1).bind(|&x| Some(x + 1)), Some(2));
    }
}
