extern crate rand;

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop0   = 0x00, // no operation
    Nop1   = 0x01, // no operation
    Or1    = 0x02, // flip low order bit of cx, cx ^= 1
    Shl    = 0x03, // shift left cx register, cx <<= 1
    Zero   = 0x04, // set cx register to zero, cx = 0
    IfCz   = 0x05, // if cx == 0 execute next instruction
    SubAb  = 0x06, // subtract bx from ax, cx = ax - bx
    SubAc  = 0x07, // subtract cx from ax, ax = ax - cx
    IncA   = 0x08, // increment ax, ax = ax + 1
    IncB   = 0x09, // increment bx, bx = bx + 1
    DecC   = 0x0A, // decrement cx, cx = cx - 1
    IncC   = 0x0B, // increment cx, cx = cx + 1
    PushAx = 0x0C, // push ax on stack
    PushBx = 0x0D, // push bx on stack
    PushCx = 0x0E, // push cx on stack
    PushDx = 0x0F, // push dx on stack
    PopAx  = 0x10, // pop top of stack into ax
    PopBx  = 0x11, // pop top of stack into bx
    PopCx  = 0x12, // pop top of stack into cx
    PopDx  = 0x13, // pop top of stack into dx
    Jmp    = 0x14, // move ip to template
    Jmpb   = 0x15, // move ip backward to template
    Call   = 0x16, // call a procedure
    Ret    = 0x17, // return from a procedure
    MovCd  = 0x18, // move cx to dx, dx = cx
    MovAb  = 0x19, // move ax to bx, bx = ax
    MovIab = 0x1A, // move instruction at address in bx to address in ax
    Adr    = 0x1B, // address of nearest template to ax
    Adrb   = 0x1C, // search backward for template
    Adrf   = 0x1D, // search forward for template
    Mal    = 0x1E, // allocate memory for daughter cell
    Divide = 0x1F, // cell division
}

impl Instruction {
    pub fn is_nop(x: Instruction) -> bool
    {
        (x == Instruction::Nop0) || (x == Instruction::Nop1)
    }

    pub fn from_usize(x: usize) -> Instruction
    {
        use Instruction::*;
        match x {
            0x00 => Nop0,
            0x01 => Nop1,
            0x02 => Or1,
            0x03 => Shl,
            0x04 => Zero,
            0x05 => IfCz,
            0x06 => SubAb,
            0x07 => SubAc,
            0x08 => IncA,
            0x09 => IncB,
            0x0A => DecC,
            0x0B => IncC,
            0x0C => PushAx,
            0x0D => PushBx,
            0x0E => PushCx,
            0x0F => PushDx,
            0x10 => PopAx,
            0x11 => PopBx,
            0x12 => PopCx,
            0x13 => PopDx,
            0x14 => Jmp,
            0x15 => Jmpb,
            0x16 => Call,
            0x17 => Ret,
            0x18 => MovCd,
            0x19 => MovAb,
            0x1A => MovIab,
            0x1B => Adr,
            0x1C => Adrb,
            0x1D => Adrf,
            0x1E => Mal,
            0x1F => Divide,
            _    => panic!("it does not match any instruction.")
        }
    }

    pub fn mutate_bit_randomly(&self) -> Instruction
    {
        let target_bit = rand::thread_rng().gen_range(0, 5);
        Instruction::from_usize((*self as usize) ^ (1 << target_bit))
    }
}

