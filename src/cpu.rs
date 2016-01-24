use bus::{Bus};
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

    pub fn tick(&mut self, bus: &mut Bus) {
        let mut pc = self.regs.read16(Reg16::PC);
        let instruction = Instruction::decode(|| {
            let word = bus.read_word(pc);
            pc += 1;
            word
        });
        println!("Got instruction: {:?}", instruction);
        self.handle_instruction(bus, instruction);
        self.regs.write16(Reg16::PC, pc);
    }

    fn handle_instruction(&mut self, bus: &mut Bus, instruction: Instruction) {
        use z80::instructions::Instruction::*;
        match instruction {
            Load8(dest, src) => {
                let val = self.read_src8(bus, src);
                self.write_dest8(bus, dest, val);
            }
            Load8Inc(dest, src) => {
                let val = self.read_src8(bus, src);
                self.write_dest8(bus, dest, val);
                let hl = self.regs.read16(Reg16::HL);
                self.regs.write16(Reg16::HL, hl+1);
            }
            Load8Dec(dest, src) => {
                let val = self.read_src8(bus, src);
                self.write_dest8(bus, dest, val);
                let hl = self.regs.read16(Reg16::HL);
                self.regs.write16(Reg16::HL, hl-1);
            }
            Load16(dest, src) => {
                let val = match src {
                    Src16::Imm(val) => val,
                    Src16::Reg(reg) => self.regs.read16(reg),
                };
                self.regs.write16(dest, val);
            }
            Push(reg) => {
                let sp = self.regs.read16(Reg16::SP) - 2;
                self.regs.write16(Reg16::SP, sp);
                bus.write_double(sp, self.regs.read16(reg));
            }
            Pop(reg) => {
                let sp = self.regs.read16(Reg16::SP);
                self.regs.write16(reg, bus.read_double(sp));
                self.regs.write16(Reg16::SP, sp-2);
            }
            Add(src) => {
                self.do_add(bus, src, false);
            }
            AddCarry(src) => {
                let carry = self.regs.carry_flag();
                self.do_add(bus, src, carry);
            }
            Sub(src) => {
                self.do_sub(bus, src, false, true);
            }
            SubCarry(src) => {
                let carry = self.regs.carry_flag();
                self.do_sub(bus, src, carry, true);
            }
            And(src) => {
                let left = self.regs.read8(Reg8::A);
                let right = self.read_src8(bus, src);
                let val = left & right;
                self.regs.write8(Reg8::A, val);
                self.regs.set_zero_flag(val == 0);
                self.regs.set_half_carry_flag(false);
                self.regs.set_sub_flag(false);
                self.regs.set_carry_flag(true);
            }
            Or(src) => {
                let left = self.regs.read8(Reg8::A);
                let right = self.read_src8(bus, src);
                let val = left | right;
                self.regs.write8(Reg8::A, val);
                self.regs.set_zero_flag(val == 0);
                self.regs.set_half_carry_flag(false);
                self.regs.set_sub_flag(false);
                self.regs.set_carry_flag(false);
            }
            Xor(src) => {
                let left = self.regs.read8(Reg8::A);
                let right = self.read_src8(bus, src);
                let val = left ^ right;
                self.regs.write8(Reg8::A, val);
                self.regs.set_zero_flag(val == 0);
                self.regs.set_half_carry_flag(false);
                self.regs.set_sub_flag(false);
                self.regs.set_carry_flag(false);
            }
            Compare(src) => {
                self.do_sub(bus, src, false, true);
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
                let pre = bus.read_word(addr);
                let post = pre+1;
                bus.write_word(addr, post);
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
                let pre = bus.read_word(addr);
                let post = pre-1;
                bus.write_word(addr, post);
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
            Unknown(opcode) => panic!("Got unknown opcode: 0x{:x}", opcode),
            _ => panic!("Unimplemented instruction: {:?}", instruction),
        }
        println!("{:?}", self);
    }

    fn read_src8(&self, bus: &mut Bus, src: Src8) -> u8 {
        match src {
            Src8::Imm(val) => val,
            Src8::Reg(reg) => self.regs.read8(reg),
            Src8::Indir(reg) => bus.read_word(self.regs.read16(reg)),
            Src8::Mem(addr) => bus.read_word(addr),
        }
    }

    fn write_dest8(&mut self, bus: &mut Bus, dest: Dest8, val: u8) {
        match dest {
            Dest8::Reg(reg) => self.regs.write8(reg, val),
            Dest8::Indir(reg) => bus.write_word(self.regs.read16(reg), val),
            Dest8::Mem(addr) => bus.write_word(addr, val),
        }
    }

    fn do_add(&mut self, bus: &mut Bus, src: Src8, carry: bool) {
        let left = self.regs.read8(Reg8::A);
        let right = self.read_src8(bus, src);
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

    fn do_sub(&mut self, bus: &mut Bus, src: Src8, carry: bool, store: bool) {
        let left = self.regs.read8(Reg8::A);
        let right = self.read_src8(bus, src);
        let carry = if carry { 1 } else { 0 };
        let val = left - right - carry;
        if store { self.regs.write8(Reg8::A, val); }
        self.regs.set_zero_flag(val == 0);
        self.regs.set_sub_flag(true);
        self.regs.set_half_carry_flag((left & 0xF) < (right & 0xF) + carry);
        self.regs.set_carry_flag(left < right + carry);
    }
}
