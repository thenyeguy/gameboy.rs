use bus::Bus;
use cpu::Cpu;

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Gameboy {
        Gameboy {
            bus: Bus::new(rom),
            cpu: Cpu::new(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        }
    }
}
