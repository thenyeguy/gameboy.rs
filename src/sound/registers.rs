use sound::tone_channel::ToneChannel;

#[derive(Debug, Default)]
pub struct SoundRegisters {
    sound_enable: SoundEnable,
    sweep_channel: ToneChannel,
    tone_channel: ToneChannel,
}

impl SoundRegisters {
    pub fn new() -> Self {
        SoundRegisters {
            sweep_channel: ToneChannel::new(true),
            tone_channel: ToneChannel::new(false),
            .. Default::default()
        }
    }

    pub fn read(&self, port: u8) -> u8 {
        match port {
            0x26 => self.sound_enable.read(),
            0x10 => self.sweep_channel.read(0),
            0x11 => self.sweep_channel.read(1),
            0x12 => self.sweep_channel.read(2),
            0x13 => self.sweep_channel.read(3),
            0x14 => self.sweep_channel.read(4),
            0x16 => self.tone_channel.read(1),
            0x17 => self.tone_channel.read(2),
            0x18 => self.tone_channel.read(3),
            0x19 => self.tone_channel.read(4),
            _ => panic!("Invalid port for SoundRegisters::read: {:#X}", port)
        }
    }

    pub fn write(&mut self, port: u8, val: u8) {
        match port {
            0x26 => self.sound_enable.write(val),
            0x10 => self.sweep_channel.write(0, val),
            0x11 => self.sweep_channel.write(1, val),
            0x12 => self.sweep_channel.write(2, val),
            0x13 => self.sweep_channel.write(3, val),
            0x14 => self.sweep_channel.write(4, val),
            0x16 => self.tone_channel.write(1, val),
            0x17 => self.tone_channel.write(2, val),
            0x18 => self.tone_channel.write(3, val),
            0x19 => self.tone_channel.write(4, val),
            _ => panic!("Invalid port for SoundRegisters::write: {:#X}", port)
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
