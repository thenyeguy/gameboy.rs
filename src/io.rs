use sound::SoundRegisters;


#[derive(Debug, Default)]
pub struct IoPorts {
    sound: SoundRegisters,
}

impl IoPorts {
    pub fn new() -> Self {
        IoPorts {
            sound: SoundRegisters::new(),
            ..Default::default()
        }
    }

    pub fn read(&self, port: u8) -> u8 {
        match port {
            0x10...0x3F => self.sound.read(port),
            _ => panic!("Invalid port for IoPort::read: {:#X}", port),
        }
    }

    pub fn write(&mut self, port: u8, val: u8) {
        match port {
            0x10...0x3F => self.sound.write(port, val),
            _ => panic!("Invalid portess for IoPort::write: {:#X}", port),
        }
    }

    pub fn sound_registers(&mut self) -> &mut SoundRegisters {
        &mut self.sound
    }
}
