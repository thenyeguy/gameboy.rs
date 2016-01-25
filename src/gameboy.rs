use mmu::MMU;
use cartridge::Cartridge;
use z80::cpu::Cpu;

pub struct Gameboy {
    mmu: MMU,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(cart: Cartridge) -> Gameboy {
        Gameboy {
            mmu: MMU::new(cart),
            cpu: Cpu::new(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.mmu);
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        }
    }
}
