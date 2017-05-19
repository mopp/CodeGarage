use std::ops::Add;

trait Monoid: Eq + Copy {
    fn append(self, x: Self) -> Self;
    fn zero(self) -> Self;
}


fn left_identity_law<T>(a: T) -> bool where T: Monoid
{
    a.zero().append(a) == a
}

fn right_identity_law<T>(a: T) -> bool where T: Monoid
{
    a.append(a.zero()) == a
}

fn associative_law<T>(a: T, b: T, c: T) -> bool where T: Monoid
{
    a.append(b).append(c) == a.append(b.append(c))
}


impl<T> Monoid for Option<T> where T: Add<Output = T> + Eq + Copy {
    fn append(self, y: Self) -> Self
    {
        match (self, y) {
            (None,     None)     => None,
            (Some(v),  None)     => Some(v),
            (None,     Some(v))  => Some(v),
            (Some(v1), Some(v2)) => Some(v1 + v2),
        }
    }

    fn zero(self) -> Self
    {
        None
    }
}


fn main()
{
    assert_eq!(true, left_identity_law(Some(1)));
    assert_eq!(true, right_identity_law(Some(2)));
    assert_eq!(true, associative_law(Some(2), Some(2), Some(10)));
    assert_eq!(true, associative_law(Some(2), Some(2), Some(10)));

    assert_eq!(Some(1).append(Some(2)), Some(3));
    assert_eq!(Some(1).append(Some(2)).append(Some(10)), Some(13));
}
