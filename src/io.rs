#[derive(Debug, Default)]
pub struct IoPorts {
    pub sound_enable: SoundEnable
}

impl IoPorts {
    pub fn new() -> Self {
        IoPorts { ..Default::default() }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF26 => self.sound_enable.read(),
            _ => panic!("Invalid address supplied to IoPort::read: {:#X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF26 => self.sound_enable.write(val),
            _ => panic!("Invalid address supplied to IoPort::write: {:#X}", addr),
        }
    }
}


#[derive(Debug, Default)]
pub struct SoundEnable {
    sound_enabled: bool,
    channel_enabled: [bool; 4],
}

impl SoundEnable {
    fn read(&self) -> u8 {
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

    fn write(&mut self, val: u8) {
        self.sound_enabled = (val >> 7) == 0;
    }
}
