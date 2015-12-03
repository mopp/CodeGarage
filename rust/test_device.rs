// Enable device module.
mod device;
// Import Device trait in the device module.
use device::Device;

struct Test {
    number:i32
}

impl Device for Test {
    fn open(&self) {
        println!("OPEN!");
    }
}


fn main()
{
    let t = Test{number: 100};

    println!("{}", t.number);
    t.open();
    t.default_method();
}
