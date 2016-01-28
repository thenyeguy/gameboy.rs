#[derive(Debug, Default)]
pub struct SoundRegisters {
    sound_enable: SoundEnable,
}

impl SoundRegisters {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF26 => self.sound_enable.read(),
            _ => panic!("Invalid address for SoundRegisters::read: {:#X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF26 => self.sound_enable.write(val),
            _ => panic!("Invalid address for SoundRegisters::write: {:#X}", addr),
        }
    }
}


#[derive(Debug, Default)]
pub struct SoundEnable {
    sound_enabled: bool,
    channel_enabled: [bool; 4],
}

impl SoundEnable {
    pub fn read(&self) -> u8 {
        let mut out = 0;
        if self.sound_enabled {
            out |= 0b1 << 7;
        }
        for i in 0..self.channel_enabled.len() {
            if self.channel_enabled[i] {
                out |= 0b1 << i;
            }
        }
        out
    }

    pub fn write(&mut self, val: u8) {
        self.sound_enabled = (val >> 7) == 0;
    }
}
