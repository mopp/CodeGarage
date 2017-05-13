// http://qiita.com/k5n/items/758111b12740600cc58f
macro_rules! conv_strs_to_strings{
    ( $($str:expr),+ ) => {
        [ $( $str.to_string() ),+ ]
    }
}


fn main()
{
    let ss = conv_strs_to_strings!["one", "two", "three"];
    println!("{:?}", ss);
}
