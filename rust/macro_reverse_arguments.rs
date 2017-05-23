// https://stackoverflow.com/questions/42171483/how-to-recursively-take-the-last-argument-of-a-macro

macro_rules! reverse {
    ($macro:ident, $( $v:tt ),* ) => {
        reverse!($macro, [$( $v )*] [])
    };

    ($macro:ident, [$head:tt $( $rest:tt )*] [$( $reversed:tt )*]) => {
        reverse!($macro, [$( $rest )*] [$head $( $reversed )*])
    };

    ($macro:ident, [] [$( $reversed:tt )*]) => {
        $macro!($( $reversed ),*)
    };
}


fn main()
{
    assert_eq!(reverse!(concat, "A ", "B ", "C "), "C B A ");
    assert_eq!(reverse!(concat, "test", 10, 'b', true), "trueb10test");
    assert_eq!(vec![1, 2, 3], reverse!(vec, 3, 2, 1));
}
