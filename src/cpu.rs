use std::default::Default;

use bus::{Bus};
use z80::instructions::Instruction;
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
        use z80::instructions::{Src8, Dest8, Src16};
        use z80::instructions::Instruction::*;
        match instruction {
            Load8(dest, src) => {
                let val = match src {
                    Src8::Imm(val) => val,
                    Src8::Reg(reg) => self.regs.read8(reg),
                    Src8::Indir(reg) => bus.read_word(self.regs.read16(reg)),
                    Src8::Mem(addr) => bus.read_word(addr),
                };
                match dest {
                    Dest8::Reg(reg) => self.regs.write8(reg, val),
                    Dest8::Indir(reg) => bus.write_word(self.regs.read16(reg), val),
                    Dest8::Mem(addr) => bus.write_word(addr, val),
                }
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
            _ => panic!("Unimplemented instruction: {:?}", instruction),
        }
        println!("{:?}", self);
    }
}
