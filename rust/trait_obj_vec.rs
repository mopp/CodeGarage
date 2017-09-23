trait Hoge {
    fn name(&self) -> String;
}


struct A(usize);
struct B(i32);
struct C(u8);

impl Hoge for A {
    fn name(&self) -> String {
        format!("I'm A: {}",self.0)
    }
}

impl Hoge for B {
    fn name(&self) -> String {
        format!("I'm B: {}",self.0)
    }
}

impl Hoge for C {
    fn name(&self) -> String {
        format!("I'm C: {}",self.0)
    }
}

fn exec(v: Vec<Box<Hoge>>) {
    for i in v.iter() {
        println!("{}", i.name());
    }
}

fn main() {
    let a = A(100);
    let b = B(-55);
    let c = C(3);

    let hoges: Vec<Box<Hoge>> = vec![Box::new(a), Box::new(b), Box::new(c)];
    exec(hoges);
}
