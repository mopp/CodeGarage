struct A(&'static str);


impl Drop for A {
    fn drop(&mut self)
    {
        println!("dropped: {}", self.0);
    }
}


fn main()
{
    let _ = A("x");
    let y = A("y");
    let z = A("z");
}


// dropped: x
// dropped: z
// dropped: y
