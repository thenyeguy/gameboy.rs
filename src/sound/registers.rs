use sound::tone_channel::ToneChannel;
use sound::wav_channel::WavChannel;

#[derive(Debug, Default)]
pub struct SoundRegisters {
    sound_enable: SoundEnable,
    sweep_channel: ToneChannel,
    tone_channel: ToneChannel,
    wav_channel: WavChannel,
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
            0x10...0x14 => self.sweep_channel.read(port - 0x10),
            0x16...0x19 => self.tone_channel.read(port - 0x16 + 1),
            0x1A...0x1E => self.wav_channel.read(port - 0x1A),
            _ => panic!("Invalid port for SoundRegisters::read: {:#X}", port)
        }
    }

    pub fn write(&mut self, port: u8, val: u8) {
        match port {
            0x26 => self.sound_enable.write(val),
            0x10...0x14 => self.sweep_channel.write(port - 0x10, val),
            0x16...0x19 => self.tone_channel.write(port - 0x16 + 1, val),
            0x1A...0x1E => self.wav_channel.write(port - 0x1A, val),
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
