extern crate docopt;
extern crate libgameboy;
extern crate rustc_serialize;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;


#[derive(Debug, RustcDecodable)]
struct Args {
    arg_rom: String,
}

const USAGE: &'static str = "
Usage: gamebody <rom>
       gamebody (-h | --help)

Options:
  -h --help     Show this screen.
";

fn load_rom<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = try!(File::open(path));
    let mut buffer = Vec::new();
    try!(file.read_to_end(&mut buffer));
    Ok(buffer)
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
                                    .and_then(|d| d.decode())
                                    .unwrap_or_else(|e| e.exit());

    println!("Loading ROM: {}", args.arg_rom);
    let rom = load_rom(args.arg_rom).expect("Failed to load ROM");
    let mut gameboy = libgameboy::Gameboy::new(rom);
    gameboy.run();
}
