use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;


pub struct Cartridge {
    data: Vec<u8>,
    title: String,
    ram_size: u16,
}

impl Cartridge {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = try!(File::open(path));
        let mut buffer = Vec::new();
        try!(file.read_to_end(&mut buffer));
        Ok(Cartridge::from_buffer(buffer))
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        let title = String::from_utf8(
            buffer[0x0134..0x0143].into_iter().cloned()
                .take_while(|b| *b != 0x00).collect()).unwrap();

        let ram_size = match buffer[0x0149] {
            0x00 => 0,
            0x01 => 2*1024,
            0x02 => 8*1024,
            0x03 => 32*1024,
            _ => panic!("Invalid RAM size"),
        };

        Cartridge {
            data: buffer,
            title: title,
            ram_size: ram_size,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

impl fmt::Debug for Cartridge {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Cartridge")
            .field("title", &self.title)
            .field("ram_size", &self.ram_size)
            .finish()
    }
}
