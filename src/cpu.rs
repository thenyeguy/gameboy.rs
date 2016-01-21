use std::default::Default;

use bus::{Bus, WORK_RAM_START};

#[derive(Debug, Default)]
pub struct Cpu {
    reg_a: u8,
    reg_f: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_h: u8,
    reg_l: u8,
    reg_sp: u16,
    reg_pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            // boot ROM loaded to start of memory
            reg_pc: WORK_RAM_START,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        let opcode = bus.read_word(self.reg_pc);
        match (opcode >> 4, opcode & 0x0F) {
            (reg, 0x1) => { // ld rr,dd
                let value: u16 = (bus.read_word(self.reg_pc+1) as u16) +
                    ((bus.read_word(self.reg_pc+2) as u16) << 8);
                self.write_register16(reg, value);
                self.reg_pc += 3;
            }
            _ => {
                panic!("unimplemented opcode: 0x{:x} 0b{:b}", opcode, opcode);
            }
        }
        println!("Tick: {:?}", self);
    }

    fn write_register16(&mut self, reg: u8, value: u16) {
        match reg {
            0b11 => self.reg_sp = value,
            _ => unreachable!()
        }
    }
}
