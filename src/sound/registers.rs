use sound::noise_channel::NoiseChannel;
use sound::tone_channel::ToneChannel;
use sound::wav_channel::WavChannel;
use utils::BitOps;

#[derive(Debug, Default)]
pub struct SoundRegisters {
    sweep_channel: ToneChannel,
    tone_channel: ToneChannel,
    wav_channel: WavChannel,
    noise_channel: NoiseChannel,
    sound_enable: SoundEnable,
    channel_control: ChannelControl,
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
            0x10...0x14 => self.sweep_channel.read(port - 0x10),
            0x16...0x19 => self.tone_channel.read(port - 0x16 + 1),
            0x1A...0x1E => self.wav_channel.read(port - 0x1A),
            0x20...0x23 => self.noise_channel.read(port - 0x20 + 1),
            0x24...0x25 => self.channel_control.read(port - 0x24),
            0x26 => self.sound_enable.read(),
            _ => panic!("Invalid port for SoundRegisters::read: {:#X}", port)
        }
    }

    pub fn write(&mut self, port: u8, val: u8) {
        match port {
            0x10...0x14 => self.sweep_channel.write(port - 0x10, val),
            0x16...0x19 => self.tone_channel.write(port - 0x16 + 1, val),
            0x1A...0x1E => self.wav_channel.write(port - 0x1A, val),
            0x20...0x23 => self.noise_channel.write(port - 0x20 + 1, val),
            0x24...0x25 => self.channel_control.write(port - 0x24, val),
            0x26 => self.sound_enable.write(val),
            _ => panic!("Invalid port for SoundRegisters::write: {:#X}", port)
        }
    }
}


#[derive(Debug, Default)]
pub struct SoundEnable {
    sound_enabled: bool,
    channel_on: [bool; 4],
}

impl SoundEnable {
    pub fn read(&self) -> u8 {
        let mut out = 0;
        if self.sound_enabled {
            out |= 0b1 << 7;
        }
        for i in 0..self.channel_on.len() {
            if self.channel_on[i] {
                out |= 0b1 << i;
            }
        }
        out
    }

    pub fn write(&mut self, val: u8) {
        self.sound_enabled = (val >> 7) == 0;
    }
}


#[derive(Debug, Default)]
pub struct ChannelControl {
    so1_volume: u8,
    so2_volume: u8,
    output_to_so1: [bool; 4],
    output_to_so2: [bool; 4],
    output_vin_to_so1: bool,
    output_vin_to_so2: bool,
}

impl ChannelControl {
    pub fn read(&self, reladdr: u8) -> u8 {
        match reladdr {
            0 => {
                let mut out = (self.so2_volume << 4) | self.so1_volume;
                out.set_bit(3, self.output_vin_to_so1);
                out.set_bit(7, self.output_vin_to_so2);
                out
            }
            1 => {
                let mut out = 0;
                for (i, output) in self.output_to_so1.iter()
                        .chain(self.output_to_so2.iter()).enumerate() {
                    out.set_bit(i as u8, *output);
                }
                out
            }
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, reladdr: u8, val: u8) {
        match reladdr {
            0 => {
                self.so1_volume = val & 0b111;
                self.so2_volume = (val >> 4) & 0b111;
                self.output_vin_to_so1 = val.get_bit(3);
                self.output_vin_to_so2 = val.get_bit(7);
            }
            1 => {
                for i in 0u8..4 {
                    self.output_to_so1[i as usize] = val.get_bit(i);
                    self.output_to_so2[i as usize] = val.get_bit(i+4);
                }
            }
            _ => unreachable!(),
        }
    }
}
