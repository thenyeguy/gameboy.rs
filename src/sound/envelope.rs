#[derive(Copy, Clone, Debug, Default)]
pub struct EnvelopeRegister {
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


#[derive(Copy, Clone, Debug)]
enum Direction { Increase, Decrease }
impl Default for Direction {
    fn default() -> Self { Direction::Increase }
}
