use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Cartridge {
    pub data: Vec<u8>
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = try!(File::open(path));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
        Ok(Cartridge { data: buffer })
    }
}
