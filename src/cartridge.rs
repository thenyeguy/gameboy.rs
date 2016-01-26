use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Cartridge {
    data: Vec<u8>
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = try!(File::open(path));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
        Ok(Cartridge { data: buffer })
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
}
