/// [A Rust macro for the builder pattern](http://jadpole.github.io/rust/builder-macro)

macro_rules! builder {
    ( $src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    })
    => {
        #[derive(Debug)]
        struct $dest_name {
            $( $attr_name : $attr_type ),*
        }

        struct $src_name {
            $( $attr_name : Option<$attr_type> ),*
        }

        impl $src_name {
            pub fn new() -> $src_name {
                $src_name {
                    $(
                        $attr_name : $attr_default
                    ),*
                }
            }

            pub fn build(self) -> Result<$dest_name, &'static str> {
                let err = "Argument missing";

                $(
                    let $attr_name = try!(self.$attr_name.ok_or(err));
                )*

                Ok($dest_name {
                    $( $attr_name : $attr_name ),*
                })
            }

            $(
                fn $attr_name(mut self, value: $attr_type) -> Self {
                    self.$attr_name = Some(value);
                    self
                }
            )*
        }
    }
}

builder!(UserBuilder => User {
    name: String = None,
    age: usize = None
});

builder!(ObjectBuilder => Object {
    name: String = Some("default name".to_string()),
    category: usize = None
});

fn main() {
    // ok
    match UserBuilder::new().name("hoge".to_string()).age(123).build() {
        Ok(u) => println!("{:?}", u),
        Err(e) => println!("{:?}", e),
    }

    // error
    match UserBuilder::new().age(123).build() {
        Ok(u) => println!("{:?}", u),
        Err(e) => println!("{:?}", e),
    }

    // ok
    match ObjectBuilder::new().category(0).build() {
        Ok(u) => println!("{:?}", u),
        Err(e) => println!("{:?}", e),
    }

    // ok
    match ObjectBuilder::new().name("not default name".to_string()).category(0).build() {
        Ok(u) => println!("{:?}", u),
        Err(e) => println!("{:?}", e),
    }
}
