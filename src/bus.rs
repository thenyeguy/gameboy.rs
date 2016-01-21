const RAM_SIZE: usize = 8*1024;

pub struct Bus {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Bus {
        Bus {
            rom: rom,
            ram: vec![0; RAM_SIZE],
        }
    }
}
