trait Parts {
    type T;
    fn lo(&self) -> Self::T;
    fn hi(&self) -> Self::T;
}


impl Parts for u16 {
    type T = u8;
    fn lo(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn hi(&self) -> u8 {
        ((self >> 8) & 0xFFu16) as u8
    }
}

fn main() {
    println!("hi 0x{:x}", 0x1010.hi());
    println!("lo 0x{:x}", 0x1010.lo());

    println!("hi 0x{:x}", 0xFF10.hi());
    println!("lo 0x{:x}", 0xFF10.lo());

    println!("hi 0x{:x}", 0x0D.hi());
    println!("lo 0x{:x}", 0x0D.lo());

    println!("hi 0x{:x}", 0x09.hi());
    println!("lo 0x{:x}", 0x09.lo());
}
