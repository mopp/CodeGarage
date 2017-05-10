// http://qiita.com/k5n/items/e95842744fc5db931d03

#[macro_use]
extern crate nom;

use nom::{IResult, space, alpha};

named!(name_parser<&str>,
    chain!(
        tag!("Hello,") ~
        space? ~
        name: map_res!(
            alpha,
            std::str::from_utf8
        ) ~
        tag!("!") ,

        || name
    )
);


fn main()
{
    match name_parser("Hello, world!".as_bytes()) {
        IResult::Done(_, name) => println!("name = {}", name),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
