use std::default::Default;

use bus::WORK_RAM_START;


#[derive(Copy, Clone, Debug)]
pub enum Reg8 { A, B, C, D, E, H, L }

#[derive(Copy, Clone, Debug)]
pub enum Reg16 { AF, BC, DE, HL, SP, PC }

#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: WORK_RAM_START,
            ..Default::default()
        }
    }

    pub fn read8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub fn read16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => ((self.a as u16) << 8) | (self.f as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
            Reg16::SP => self.sp,
            Reg16::PC => self.pc,
        }
    }

    pub fn write8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.a = val,
            Reg8::B => self.b = val,
            Reg8::C => self.c = val,
            Reg8::D => self.d = val,
            Reg8::E => self.e = val,
            Reg8::H => self.h = val,
            Reg8::L => self.l = val,
        }
    }

    pub fn write16(&mut self, reg: Reg16, val: u16) {
        match reg {
            Reg16::AF => {
                self.a = (val >> 8) as u8;
                self.f = (val & 0xFF) as u8;
            }
            Reg16::BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0xFF) as u8;
            }
            Reg16::DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0xFF) as u8;
            }
            Reg16::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0xFF) as u8;
            }
            Reg16::SP => self.sp = val,
            Reg16::PC => self.pc = val,
        }
    }

    pub fn zero_flag(&self) -> bool {
        self.get_flag(7)
    }

    pub fn set_zero_flag(&mut self, val: bool) {
        self.set_flag(7, val);
    }

    pub fn sub_flag(&self) -> bool {
        self.get_flag(6)
    }

    pub fn set_sub_flag(&mut self, val: bool) {
        self.set_flag(6, val);
    }

    pub fn half_carry_flag(&self) -> bool {
        self.get_flag(5)
    }

    pub fn set_half_carry_flag(&mut self, val: bool) {
        self.set_flag(5, val);
    }

    pub fn carry_flag(&self) -> bool {
        self.get_flag(4)
    }

    pub fn set_carry_flag(&mut self, val: bool) {
        self.set_flag(4, val);
    }

    fn get_flag(&self, digit: u8) -> bool {
        ((self.f >> digit) & 0x1) == 1
    }

    fn set_flag(&mut self, digit: u8, val: bool) {
        if val {
            self.f = self.f | (0x1 << digit);
        } else {
            self.f = self.f & !(0x1 << digit);
        }
    }
}
