#[macro_use]
extern crate nom;

use nom::{IResult, be_u8};

named!(factor<i64>,
    map_res!(
        map_res!(
            ws!(nom::digit),
            std::str::from_utf8
        ),
        std::str::FromStr::from_str
    )
);


fn main()
{
    print_parser_result(factor("129".as_bytes()));
}


fn print_parser_result<T, U>(r: IResult<T, U>) where T: std::fmt::Debug, U: std::fmt::Debug
{
    match r {
        IResult::Done(remain, get)  => println!("remain = {:?}, get = {:?}", remain, get),
        IResult::Error(error)       => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
