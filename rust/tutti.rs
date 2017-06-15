type Address  = u16;
type Register = u16;


struct cpu {
    sp: Register,
    ip: Register,
    registers: [Register; 4],
    flags: u8,
    stack: [Address; 10],
}


fn main()
{
}
