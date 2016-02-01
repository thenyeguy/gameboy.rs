use sound::envelope::EnvelopeRegister;
use utils::{BitOps};

#[derive(Default, Debug)]
pub struct NoiseChannel {
    sound_length: u8,
    envelope: EnvelopeRegister,
    shift_clock_frequency: u8,
    regularity: Regularity,
    dividing_ratio: u8,
    restart_sound: bool,
    use_sound_length: bool,
}

impl NoiseChannel {
    pub fn read(&self, reladdr: u8) -> u8 {
        match reladdr {
            1 => self.sound_length,
            2 => self.envelope.into(),
            3 => {
                let mut out = self.shift_clock_frequency << 4;
                if let Regularity::Regular = self.regularity {
                    out.set_bit(3, true);
                }
                out |= self.dividing_ratio;
                out
            }
            4 => {
                let mut out = 0;
                out.set_bit(6, self.use_sound_length);
                out.set_bit(7, self.restart_sound);
                out
            }
            _ => panic!("Invalid addr for WavChannel::read: {:#X}", reladdr)
        }
    }

    pub fn write(&mut self, reladdr: u8, val: u8) {
        match reladdr {
            1 => self.sound_length = val,
            2 => self.envelope = val.into(),
            3 => {
                self.shift_clock_frequency = (val >> 4) & 0b1111;
                self.regularity = if val.get_bit(3) {
                    Regularity::Regular
                } else {
                    Regularity::Irregular
                };
                self.dividing_ratio = val & 0b111;
            }
            4 => {
                self.use_sound_length = val.get_bit(6);
                self.restart_sound = val.get_bit(7);
            }
            _ => panic!("Invalid addr for WavChannel::write: {:#X}", reladdr)
        }
    }
}


#[derive(Copy, Clone, Debug)]
enum Regularity { Regular, Irregular }
impl Default for Regularity {
    fn default() -> Self { Regularity::Irregular }
}
