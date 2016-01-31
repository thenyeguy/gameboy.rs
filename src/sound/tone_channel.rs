use utils::{BitOps, WordOps};

// TODO: update the parsing into useful values, not just bytes
#[derive(Default, Debug)]
pub struct ToneChannel {
    wave_duty: u8,
    sound_length: u8,
    frequency: u16,
    restart_sound: bool,
    use_sound_length: bool,
    envelope: EnvelopeRegister,
    sweep: Option<SweepRegister>,
}

impl ToneChannel {
    pub fn new(with_sweep: bool) -> Self {
        let sweep = if with_sweep { Some(SweepRegister::default()) } else { None };
        ToneChannel {
            sweep: sweep,
            ..Default::default()
        }
    }

    pub fn read(&self, reladdr: u8) -> u8 {
        match reladdr {
            0 if self.sweep.is_some() => self.sweep.unwrap().into(),
            1 => (self.wave_duty << 6) | (self.sound_length),
            2 => self.envelope.into(),
            3 => self.frequency.get_lower(),
            4 => {
                let mut out = self.frequency.get_upper();
                out.set_bit(6, self.use_sound_length);
                out.set_bit(7, self.restart_sound);
                out
            }
            _ => panic!("Invalid addr for ToneChannel::read: {:#X}", reladdr)
        }
    }

    pub fn write(&mut self, reladdr: u8, val: u8) {
        match reladdr {
            0 if self.sweep.is_some() => self.sweep = Some(val.into()),
            1 => {
                self.wave_duty = val >> 6;
                self.sound_length = val & 0b1_1111;
            }
            2 => self.envelope = val.into(),
            3 => self.frequency.set_lower(val),
            4 => {
                self.frequency.set_upper(val);
                self.use_sound_length = val.get_bit(6);
                self.restart_sound = val.get_bit(7);
            }
            _ => panic!("Invalid addr for ToneChannel::write: {:#X}", reladdr)
        }

    }
}


#[derive(Copy, Clone, Debug, Default)]
struct EnvelopeRegister {
    initial_volume: u8,
    direction: Direction,
    step_size: u8,
}

impl From<u8> for EnvelopeRegister {
    fn from(byte: u8) -> Self {
        EnvelopeRegister {
            initial_volume: (byte >> 4) & 0b111,
            direction: if byte >> 3 & 0b1 == 1 {
                           Direction::Decrease
                       } else {
                           Direction::Increase
                       },
            step_size: byte & 0b111,
        }
    }
}

impl Into<u8> for EnvelopeRegister {
    fn into(self) -> u8 {
        let mut out = 0;
        out |= self.initial_volume << 4;
        match self.direction {
            Direction::Decrease => out |= 0x8,
            Direction::Increase => (),
        }
        out |= self.step_size;
        out
    }
}


#[derive(Copy, Clone, Debug, Default)]
struct SweepRegister {
    sweep_time: u8,
    direction: Direction,
    sweep_shift: u8,
}

impl From<u8> for SweepRegister {
    fn from(byte: u8) -> Self {
        SweepRegister {
            sweep_time: (byte >> 4) & 0b111,
            direction: if byte >> 3 & 0b1 == 1 {
                           Direction::Decrease
                       } else {
                           Direction::Increase
                       },
            sweep_shift: byte & 0b111,
        }
    }
}

impl Into<u8> for SweepRegister {
    fn into(self) -> u8 {
        let mut out = 0;
        out |= self.sweep_time << 4;
        match self.direction {
            Direction::Decrease => out |= 0x8,
            Direction::Increase => (),
        }
        out |= self.sweep_shift;
        out
    }
}


#[derive(Copy, Clone, Debug)]
enum Direction { Increase, Decrease }
impl Default for Direction {
    fn default() -> Self { Direction::Increase }
}
