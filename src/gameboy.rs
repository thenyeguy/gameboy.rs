use mmu::MMU;
use cpu::Cpu;

pub struct Gameboy {
    mmu: MMU,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Gameboy {
        Gameboy {
            mmu: MMU::new(rom),
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
