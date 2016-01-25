use mmu::MMU;
use z80::instructions::{Instruction, Src8, Dest8, Src16};
use z80::registers::{Reg8, Reg16, Registers};

#[derive(Debug, Default)]
pub struct Cpu {
    regs: Registers
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: Registers::new()
        }
    }

    pub fn tick(&mut self, mmu: &mut MMU) {
        let mut pc = self.regs.read16(Reg16::PC);
        let instruction = Instruction::decode(|| {
            let word = mmu.read_word(pc);
            pc += 1;
            word
        });
        println!("Got instruction: {:?}", instruction);
        self.handle_instruction(mmu, instruction);
        self.regs.write16(Reg16::PC, pc);
    }

    fn handle_instruction(&mut self, mmu: &mut MMU, instruction: Instruction) {
        use z80::instructions::Instruction::*;
        match instruction {
            Load8(dest, src) => {
                let val = self.read_src8(mmu, src);
                self.write_dest8(mmu, dest, val);
            }
            Load8Inc(dest, src) => {
                let val = self.read_src8(mmu, src);
                self.write_dest8(mmu, dest, val);
                let hl = self.regs.read16(Reg16::HL);
                self.regs.write16(Reg16::HL, hl+1);
            }
            Load8Dec(dest, src) => {
                let val = self.read_src8(mmu, src);
                self.write_dest8(mmu, dest, val);
                let hl = self.regs.read16(Reg16::HL);
                self.regs.write16(Reg16::HL, hl-1);
            }
            Load16(dest, src) => {
                let val = match src {
                    Src16::Imm(val) => val,
                    Src16::Reg(reg) => self.regs.read16(reg),
                    Src16::SPOffset(offset) => {
                        let sp = self.regs.read16(Reg16::SP);
                        self.regs.set_half_carry_flag(
                            (sp & 0xF) + (offset as u8 as u16) > 0xF);
                        self.regs.set_carry_flag(
                            (sp & 0xFF) + (offset as u16) > 0xFF);
                        ((sp as i16) + (offset as i16)) as u16
                    }
                };
                self.regs.write16(dest, val);
            }
            Push(reg) => {
                let sp = self.regs.read16(Reg16::SP) - 2;
                self.regs.write16(Reg16::SP, sp);
                mmu.write_double(sp, self.regs.read16(reg));
            }
            Pop(reg) => {
                let sp = self.regs.read16(Reg16::SP);
                self.regs.write16(reg, mmu.read_double(sp));
                self.regs.write16(Reg16::SP, sp-2);
            }
            Add(src) => {
                self.do_add(mmu, src, false);
            }
            AddCarry(src) => {
                let carry = self.regs.carry_flag();
                self.do_add(mmu, src, carry);
            }
            Sub(src) => {
                self.do_sub(mmu, src, false, true);
            }
            SubCarry(src) => {
                let carry = self.regs.carry_flag();
                self.do_sub(mmu, src, carry, true);
            }
            And(src) => {
                let left = self.regs.read8(Reg8::A);
                let right = self.read_src8(mmu, src);
                let val = left & right;
                self.regs.write8(Reg8::A, val);
                self.regs.set_zero_flag(val == 0);
                self.regs.set_half_carry_flag(false);
                self.regs.set_sub_flag(false);
                self.regs.set_carry_flag(true);
            }
            Or(src) => {
                let left = self.regs.read8(Reg8::A);
                let right = self.read_src8(mmu, src);
                let val = left | right;
                self.regs.write8(Reg8::A, val);
                self.regs.set_zero_flag(val == 0);
                self.regs.set_half_carry_flag(false);
                self.regs.set_sub_flag(false);
                self.regs.set_carry_flag(false);
            }
            Xor(src) => {
                let left = self.regs.read8(Reg8::A);
                let right = self.read_src8(mmu, src);
                let val = left ^ right;
                self.regs.write8(Reg8::A, val);
                self.regs.set_zero_flag(val == 0);
                self.regs.set_half_carry_flag(false);
                self.regs.set_sub_flag(false);
                self.regs.set_carry_flag(false);
            }
            Compare(src) => {
                self.do_sub(mmu, src, false, true);
            }
            Increment(Dest8::Reg(reg)) => {
                let pre = self.regs.read8(reg);
                let post = pre+1;
                self.regs.write8(reg, post);
                self.regs.set_zero_flag(post == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag((pre & 0xF) + 1 > 0xF);
            }
            Increment(Dest8::Indir(reg)) => {
                let addr = self.regs.read16(reg);
                let pre = mmu.read_word(addr);
                let post = pre+1;
                mmu.write_word(addr, post);
                self.regs.set_zero_flag(post == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag((pre & 0xF) + 1 > 0xF);
            }
            Decrement(Dest8::Reg(reg)) => {
                let pre = self.regs.read8(reg);
                let post = pre-1;
                self.regs.write8(reg, post);
                self.regs.set_zero_flag(post == 0);
                self.regs.set_sub_flag(true);
                self.regs.set_half_carry_flag((pre & 0xF) + 1 > 0xF);
            }
            Decrement(Dest8::Indir(reg)) => {
                let addr = self.regs.read16(reg);
                let pre = mmu.read_word(addr);
                let post = pre-1;
                mmu.write_word(addr, post);
                self.regs.set_zero_flag(post == 0);
                self.regs.set_sub_flag(true);
                self.regs.set_half_carry_flag((pre & 0xF) < 1);
            }
            DecimalAdjust => {
                let mut a = self.regs.read8(Reg8::A);
                if self.regs.sub_flag() {
                    if self.regs.half_carry_flag() {
                        a -= 0x06;
                    }
                    if self.regs.carry_flag() {
                        a -= 0x60;
                    }
                } else {
                    if (a & 0x0F) > 0x09 || self.regs.half_carry_flag() {
                        a += 0x06;
                    }
                    if a > 0x90 || self.regs.carry_flag() {
                        a += 0x60;
                        self.regs.set_carry_flag(true);
                    }
                }
                self.regs.write8(Reg8::A, a);
                self.regs.set_zero_flag(a == 0);
                self.regs.set_half_carry_flag(false);
            }
            Complement => {
                let val = !self.regs.read8(Reg8::A);
                self.regs.write8(Reg8::A, val);
                self.regs.set_sub_flag(true);
                self.regs.set_half_carry_flag(true);
            }
            Add16(reg, Src16::Reg(src)) => {
                let left = self.regs.read16(reg);
                let right = self.regs.read16(reg);
                let val = left + right;
                self.regs.write16(reg, val);
                self.regs.set_half_carry_flag(
                    (left & 0xFFF) + (right & 0xFFF) > 0xFFF);
                self.regs.set_carry_flag(
                    (left as u32) + (right as u32) > 0xFFFF);
            }
            Add16(Reg16::SP, Src16::SPOffset(offset)) => {
                let sp = self.regs.read16(Reg16::SP);
                let val = ((sp as i16) + (offset as i16)) as u16;
                self.regs.write16(Reg16::SP, val);
                self.regs.set_half_carry_flag(
                    (sp & 0xF) + (offset as u8 as u16) > 0xF);
                self.regs.set_carry_flag(
                    (sp & 0xFF) + (offset as u16) > 0xFF);
            }
            Increment16(reg) => {
                let val = self.regs.read16(reg);
                self.regs.write16(reg, val+1);
            }
            Decrement16(reg) => {
                let val = self.regs.read16(reg);
                self.regs.write16(reg, val-1);
            }
            RotateLeftA => {
                let val = self.regs.read8(Reg8::A);
                let top = val >> 7;
                self.regs.write8(Reg8::A, val<<1 | top);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(top == 0b1);
            }
            RotateLeftACarry => {
                let val = self.regs.read8(Reg8::A);
                let top = val >> 7;
                let carry = if self.regs.carry_flag() { 1 } else { 0 };
                self.regs.write8(Reg8::A, val<<1 | carry);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(top == 0b1);
            }
            RotateRightA => {
                let val = self.regs.read8(Reg8::A);
                let bottom = val & 0b1;
                self.regs.write8(Reg8::A, val>>1 | bottom<<7);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(bottom == 0b1);
            }
            RotateRightACarry => {
                let val = self.regs.read8(Reg8::A);
                let bottom = val & 0b1;
                let carry = if self.regs.carry_flag() { 1 } else { 0 };
                self.regs.write8(Reg8::A, val>>1 | carry<<7);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(bottom == 0b1);
            }
            RotateLeft(dest) => {
                let val = self.read_dest8(mmu, dest);
                let top = val>>7;
                let out = val<<1 | top;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(top == 0b1);
            }
            RotateLeftCarry(dest) => {
                let val = self.read_dest8(mmu, dest);
                let top = val>>7;
                let carry = if self.regs.carry_flag() { 1 } else { 0 };
                let out = val<<1 | carry;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(top == 0b1);
            }
            RotateRight(dest) => {
                let val = self.read_dest8(mmu, dest);
                let bottom = val & 0b1;
                let out = val>>1 | bottom<<7;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(bottom == 0b1);
            }
            RotateRightCarry(dest) => {
                let val = self.read_dest8(mmu, dest);
                let bottom = val & 0b1;
                let carry = if self.regs.carry_flag() { 1 } else { 0 };
                let out = val>>1 | carry<<7;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(bottom == 0b1);
            }
            ShiftLeft(dest) => {
                let val = self.read_dest8(mmu, dest);
                let top = val>>7;
                let out = val<<1;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(top == 0b1);
            }
            ShiftRightLogical(dest) => {
                let val = self.read_dest8(mmu, dest);
                let bottom = val & 0b1;
                let out = val>>1;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(bottom == 0b1);
            }
            ShiftRightArithmetic(dest) => {
                let val = self.read_dest8(mmu, dest);
                let bottom = val & 0b1;
                let top = val & 0x80;
                let out = val>>1 | top;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(bottom == 0b1);
            }
            Swap(dest) => {
                let val = self.read_dest8(mmu, dest);
                let out = val>>4 | val<<4;
                self.write_dest8(mmu, dest, out);
                self.regs.set_zero_flag(out == 0);
                self.regs.set_sub_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(false);
            }
            Unknown(opcode, bitcode) =>
                panic!("Got unknown opcode: 0x{:x}_{:x}", opcode, bitcode),
            _ => panic!("Unimplemented instruction: {:?}", instruction),
        }
        println!("{:?}", self);
    }

    fn read_src8(&self, mmu: &mut MMU, src: Src8) -> u8 {
        match src {
            Src8::Imm(val) => val,
            Src8::Reg(reg) => self.regs.read8(reg),
            Src8::Indir(reg) => mmu.read_word(self.regs.read16(reg)),
            Src8::Mem(addr) => mmu.read_word(addr),
        }
    }

    fn read_dest8(&mut self, mmu: &mut MMU, dest: Dest8) -> u8 {
        match dest {
            Dest8::Reg(reg) => self.regs.read8(reg),
            Dest8::Indir(reg) => mmu.read_word(self.regs.read16(reg)),
            Dest8::Mem(addr) => mmu.read_word(addr),
        }
    }

    fn write_dest8(&mut self, mmu: &mut MMU, dest: Dest8, val: u8) {
        match dest {
            Dest8::Reg(reg) => self.regs.write8(reg, val),
            Dest8::Indir(reg) => mmu.write_word(self.regs.read16(reg), val),
            Dest8::Mem(addr) => mmu.write_word(addr, val),
        }
    }

    fn do_add(&mut self, mmu: &mut MMU, src: Src8, carry: bool) {
        let left = self.regs.read8(Reg8::A);
        let right = self.read_src8(mmu, src);
        let carry = if carry { 1 } else { 0 };
        let val = left + right + carry;
        self.regs.write8(Reg8::A, val);
        self.regs.set_zero_flag(val == 0);
        self.regs.set_sub_flag(false);
        self.regs.set_half_carry_flag(
            ((left & 0xF) + (right & 0xF) + carry) > 0xF);
        self.regs.set_carry_flag(
            ((left as u16) + (right as u16) + (carry as u16)) > 0xFF);
    }

    fn do_sub(&mut self, mmu: &mut MMU, src: Src8, carry: bool, store: bool) {
        let left = self.regs.read8(Reg8::A);
        let right = self.read_src8(mmu, src);
        let carry = if carry { 1 } else { 0 };
        let val = left - right - carry;
        if store { self.regs.write8(Reg8::A, val); }
        self.regs.set_zero_flag(val == 0);
        self.regs.set_sub_flag(true);
        self.regs.set_half_carry_flag((left & 0xF) < (right & 0xF) + carry);
        self.regs.set_carry_flag(left < right + carry);
    }
}
