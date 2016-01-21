use std::default::Default;

use bus::{Bus};
use z80::instructions::Instruction;
use z80::instructions::Instruction::*;
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
        match instruction {
            _ => panic!("Unimplemented instruction: {:?}", instruction),
        }
        self.regs.write16(Reg16::PC, pc);
        println!("{:?}", self);
    }
}
