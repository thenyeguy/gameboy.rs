pub trait BitOps {
    fn get_bit(&self, bit: u8) -> bool;
    fn set_bit(&mut self, bit: u8, val: bool);
}

impl BitOps for u8 {
    fn get_bit(&self, bit: u8) -> bool {
        assert!(bit < 8);
        (self >> bit) & 0b1 == 1
    }

    fn set_bit(&mut self, bit: u8, val: bool) {
        assert!(bit < 8);
        if val {
            *self |= 1 << bit;
        } else {
            *self &= !(1 << bit);
        }
    }
}

impl BitOps for u16 {
    fn get_bit(&self, bit: u8) -> bool {
        assert!(bit < 16);
        (self >> bit) & 0b1 == 1
    }

    fn set_bit(&mut self, bit: u8, val: bool) {
        assert!(bit < 16);
        if val {
            *self |= 1 << bit;
        } else {
            *self &= !(1 << bit);
        }
    }
}


pub trait WordOps {
    fn get_lower(&self) -> u8;
    fn get_upper(&self) -> u8;
    fn set_lower(&mut self, val: u8);
    fn set_upper(&mut self, val: u8);
}

impl WordOps for u16 {
    fn get_lower(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn get_upper(&self) -> u8 {
        (self >> 8) as u8
    }

    fn set_lower(&mut self, val: u8) {
        *self = (*self & 0xF0) | (val as u16);
    }

    fn set_upper(&mut self, val: u8) {
        *self = (*self & 0x0F) | ((val as u16) << 8);
    }
}
