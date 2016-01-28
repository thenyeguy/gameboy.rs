use sound::registers::SoundRegisters;


#[derive(Debug, Default)]
pub struct IoPorts {
    sound: SoundRegisters,
}

impl IoPorts {
    pub fn new() -> Self {
        IoPorts { ..Default::default() }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF10...0xFF3F => self.sound.read(addr),
            _ => panic!("Invalid address for IoPort::read: {:#X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF10...0xFF3F => self.sound.write(addr, val),
            _ => panic!("Invalid address for IoPort::write: {:#X}", addr),
        }
    }

    pub fn sound_registers(&mut self) -> &mut SoundRegisters {
        &mut self.sound
    }
}
