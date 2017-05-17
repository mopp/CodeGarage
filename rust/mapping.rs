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
    pub trait HKT<U> {
        type C; // Current type
        type T; // Type with C swapped with U
    }


    pub trait Functor<U> {
        type C;
        type T;
        fn map<F>(&self, f: F) -> Self::T where F: Fn(&Self::C) -> U;
    }


    impl<T, U> HKT<U> for Option<T> {
        type C = T;
        type T = Option<U>;
    }


    pub fn test()
    {
        let to_string = |x: usize| x.to_string();
        let opt_usize: Option<usize> = Some(100);
        assert_eq!(opt_usize.map(to_string), Some("100".to_string()));
    }
}
