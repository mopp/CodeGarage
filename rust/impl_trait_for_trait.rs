trait Base {
    fn info(&self) -> String;
}

impl<T> Base for T where T: Clone + Send + Sync {
    fn info(&self) -> String {
        "impl base for T".to_string()
    }
}

trait SubX {}
impl Base for Box<SubX> {
    fn info(&self) -> String {
        "impl Base for Box<SubX>".to_string()
    }
}

impl<'a> Base for &'a SubX {
    fn info(&self) -> String {
        "impl<'a> Base for &'a SubX".to_string()
    }
}

struct ObjZ;
impl SubX for ObjZ {}

fn print_base<T: Base>(x: T) {
    println!("{}", x.info())
}

fn main() {
    let foo = 10;
    print_base(foo);

    let z = ObjZ;
    let bar: Box<SubX> = Box::new(z);
    print_base(bar);

    let baz: &SubX = &ObjZ;
    print_base(baz);
}
