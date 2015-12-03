pub trait Device {
    fn open(&self);
    fn default_method(&self)
    {
        println!("default method is called !!");
    }
}
