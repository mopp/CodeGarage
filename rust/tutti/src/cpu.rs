use std::fmt;


pub type Register = u16;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpu {
    pub ax: Register,
    pub bx: Register,
    pub cx: Register,
    pub dx: Register,
    pub sp: Register,
    pub ip: Register,
    pub flags: u8,
    pub stack: [Register; 10],
    pub count_fails: usize,
}


impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s1 = format!("ax = 0x{:04X}, bx = 0x{:04X}, cx = 0x{:04X}, dx = 0x{:04X}, ", self.ax, self.bx, self.cx, self.dx);
        let s2 = format!("ip = 0x{:04X}, sp = 0x{:04X}, ", self.ip, self.sp);
        let s3 = format!("flags = 0x{:02X}", self.flags);

        write!(f, "{}{}{}", s1, s2, s3)
    }
}


impl Cpu {
    pub fn new() -> Cpu
    {
        Cpu {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            sp: 0,
            ip: 0,
            flags: 0,
            stack: [0; 10],
            count_fails: 0,
        }
    }

    pub fn count_up_fails(&mut self)
    {
        self.count_fails += 1;
    }

    pub fn push(&mut self, v: Register)
    {
        if self.stack.len() <= self.sp as usize {
            self.count_up_fails();
            return
        }

        self.stack[self.sp as usize] = v;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Option<Register>
    {
        if self.sp == 0 {
            self.count_up_fails();
            None
        } else {
            self.sp -= 1;
            Some(self.stack[self.sp as usize])
        }
    }
}
