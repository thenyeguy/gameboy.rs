extern crate docopt;
extern crate libgameboy;
extern crate rustc_serialize;


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


fn main() {
    use libgameboy::{Cartridge, Gameboy};

    let args: Args = docopt::Docopt::new(USAGE)
                                    .and_then(|d| d.decode())
                                    .unwrap_or_else(|e| e.exit());

    println!("Loading ROM: {}", args.arg_rom);
    let cart = Cartridge::from_file(args.arg_rom).expect("Failed to load ROM");
    println!("Loaded ROM with title: {}", cart.title());
    let mut gameboy = Gameboy::new(cart);
    println!("Running...\n");
    gameboy.run();
}
