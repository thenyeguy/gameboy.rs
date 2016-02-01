use utils::{BitOps, WordOps};

#[derive(Default, Debug)]
pub struct WavChannel {
    enabled: bool,
    sound_length: u8,
    level: OutputLevel,
    frequency: u16,
    restart_sound: bool,
    use_sound_length: bool,
}

impl WavChannel {
    pub fn read(&self, reladdr: u8) -> u8 {
        match reladdr {
            0 => {
                let mut out = 0;
                out.set_bit(7, self.enabled);
                out
            }
            1 => self.sound_length,
            2 => self.level.into(),
            3 => self.frequency.get_lower(),
            4 => {
                let mut out = self.frequency.get_upper();
                out.set_bit(6, self.use_sound_length);
                out.set_bit(7, self.restart_sound);
                out
            }
            _ => panic!("Invalid addr for WavChannel::read: {:#X}", reladdr)
        }
    }

    pub fn write(&mut self, reladdr: u8, val: u8) {
        match reladdr {
            0 => self.enabled = val.get_bit(7),
            1 => self.sound_length = val,
            2 => self.level = val.into(),
            3 => self.frequency.set_lower(val),
            4 => {
                self.frequency.set_upper(val);
                self.use_sound_length = val.get_bit(6);
                self.restart_sound = val.get_bit(7);
            }
            _ => panic!("Invalid addr for WavChannel::write: {:#X}", reladdr)
        }
    }
}


#[derive(Copy, Clone, Debug)]
enum OutputLevel {
    Muted, Full, Half, Quarter,
}

impl Default for OutputLevel {
    fn default() -> Self {
        OutputLevel::Muted
    }
}

impl From<u8> for OutputLevel {
    fn from(byte: u8) -> Self {
        match (byte >> 5) & 0b11 {
            0 => OutputLevel::Muted,
            1 => OutputLevel::Full,
            2 => OutputLevel::Half,
            3 => OutputLevel::Quarter,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for OutputLevel {
    fn into(self) -> u8 {
        match self {
            OutputLevel::Muted => 0,
            OutputLevel::Full => 1 << 5,
            OutputLevel::Half => 2 << 5,
            OutputLevel::Quarter => 3 << 5,
        }
    }
}
