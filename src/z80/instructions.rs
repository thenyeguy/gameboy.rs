use bus::Bus;
use z80::registers::{Reg8, Reg16};


pub enum Src8 {
    Imm(u8),
    Reg(Reg8),
    Indir(Reg16),
    Mem(u16),
}

fn src_reg8(opcode: u8) -> Src8 {
    Src8::Reg(byte_to_reg8(opcode & 0b111))
}

fn src_mem(lower: u8, upper: u8) -> Src8 {
    Src8::Mem(u16_val(lower, upper))
}


pub enum Dest8 {
    Reg(Reg8),
    Indir(Reg16),
    Mem(u16),
}

fn dest_reg8(opcode: u8) -> Dest8 {
    Dest8::Reg(byte_to_reg8(opcode>>3 & 0b111))
}

fn dest_mem(lower: u8, upper: u8) -> Dest8 {
    Dest8::Mem(u16_val(lower, upper))
}


#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Load8(Dest8, Src8),
    Unknown(u8),
}

impl Instruction {
    pub fn decode<F>(mut read_word: F) -> Self where F: FnMut() -> u8 {
        use self::Instruction::*;
        use z80::registers::Reg8::*;
        use z80::registers::Reg16::*;

        let opcode = read_word();
        match bits(opcode) {
            _ => Unknown(opcode),
        }
    }
}

fn bits(n: u8) -> (u8,u8,u8,u8,u8,u8,u8,u8) {
    (n >> 7 & 1,
     n >> 6 & 1,
     n >> 5 & 1,
     n >> 4 & 1,
     n >> 3 & 1,
     n >> 2 & 1,
     n >> 1 & 1,
     n >> 0 & 1)
}

fn reg16(opcode: u8) -> Reg16 {
    byte_to_reg16(opcode>>4 & 0b11)
}

fn byte_to_reg8(b: u8) -> Reg8 {
    match b {
        0b111 => Reg8::A,
        0b000 => Reg8::B,
        0b001 => Reg8::C,
        0b010 => Reg8::D,
        0b011 => Reg8::E,
        0b100 => Reg8::H,
        0b101 => Reg8::L,
        _ => unreachable!(),
    }
}

fn byte_to_reg16(b: u8) -> Reg16 {
    match b {
        0b00 => Reg16::BC,
        0b01 => Reg16::DE,
        0b10 => Reg16::HL,
        0b11 => Reg16::SP,
        _ => unreachable!(),
    }
}

fn u16_val(lower: u8, upper: u8) -> u16 {
    (lower as u16) + ((upper as u16) << 8)
}
