use bootrom::DEFAULT_BOOT_ROM;
use cartridge::Cartridge;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0xA000;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xE000;

pub struct MMU {
    cart: Cartridge,
    wram: Vec<u8>,
    vram: Vec<u8>,
}

impl MMU {
    pub fn new(cart: Cartridge) -> Self {
        // Map boot image into RAM for simplicity
        let mut wram = vec![0; (WRAM_END-WRAM_START) as usize];
        for (i, byte) in DEFAULT_BOOT_ROM.iter().enumerate() {
            wram[i] = *byte;
        }
        let vram = vec![0; (VRAM_END-VRAM_START) as usize];

        MMU {
            cart: cart,
            wram: wram,
            vram: vram,
        }
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        if VRAM_START <= addr && addr < VRAM_END {
            self.vram[(addr - VRAM_START) as usize]
        } else if WRAM_START <= addr && addr < WRAM_END {
            self.wram[(addr - WRAM_START) as usize]
        } else {
            panic!("SEGFAULT: bus.read_word({} (0x{:x}))", addr, addr);
        }
    }

    pub fn write_word(&mut self, addr: u16, val: u8) {
        if VRAM_START <= addr && addr < VRAM_END {
            self.vram[(addr - VRAM_START) as usize] = val;
        } else if WRAM_START <= addr && addr < WRAM_END {
            self.wram[(addr - WRAM_START) as usize] = val;
        } else {
            panic!("SEGFAULT: bus.write_word({} (0x{:x}), {})", addr, addr, val);
        }
    }

    pub fn read_double(&self, addr: u16) -> u16 {
        (self.read_word(addr) as u16) + ((self.read_word(addr+1) as u16) << 8)
    }

    pub fn write_double(&mut self, addr: u16, val: u16) {
        self.write_word(addr, (val & 0xFF) as u8);
        self.write_word(addr+1, (val>>8 & 0xFF) as u8);
    }
}
