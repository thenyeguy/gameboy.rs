use bootrom::DEFAULT_BOOT_ROM;
use cartridge::Cartridge;


pub struct MMU {
    cart: Cartridge,
    bootrom: Vec<u8>,
    wram: Vec<u8>,
    vram: Vec<u8>,
}

impl MMU {
    pub fn new(cart: Cartridge) -> Self {
        MMU {
            cart: cart,
            bootrom: Vec::from(&DEFAULT_BOOT_ROM[..]),
            wram: vec![0; (WRAM_END-WRAM_START) as usize],
            vram: vec![0; (VRAM_END-VRAM_START) as usize],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        if BOOTROM_START <= addr && addr < BOOTROM_END { // TODO check flag
            self.bootrom[(addr - BOOTROM_START) as usize]
        } else if CARTRIDGE_ROM_START <= addr && addr < CARTRIDGE_ROM_END {
            self.cart.read8(addr - CARTRIDGE_ROM_START)
        } else if VRAM_START <= addr && addr < VRAM_END {
            self.vram[(addr - VRAM_START) as usize]
        } else if WRAM_START <= addr && addr < WRAM_END {
            self.wram[(addr - WRAM_START) as usize]
        } else {
            panic!("SEGFAULT: bus.read_word({} (0x{:x}))", addr, addr);
        }
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        if VRAM_START <= addr && addr < VRAM_END {
            self.vram[(addr - VRAM_START) as usize] = val;
        } else if WRAM_START <= addr && addr < WRAM_END {
            self.wram[(addr - WRAM_START) as usize] = val;
        } else {
            panic!("SEGFAULT: bus.write_word({} (0x{:x}), {})", addr, addr, val);
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        (self.read8(addr) as u16) + ((self.read8(addr+1) as u16) << 8)
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        self.write8(addr, (val & 0xFF) as u8);
        self.write8(addr+1, (val>>8 & 0xFF) as u8);
    }
}


pub const BOOTROM_START: u16 = 0x0000;
pub const BOOTROM_END: u16 = 0x0100;

pub const CARTRIDGE_ROM_START: u16 = 0x0000;
pub const CARTRIDGE_ROM_END: u16 = 0x4000;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0xA000;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xE000;
