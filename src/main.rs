#![allow(dead_code,unused_variables)]

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate portaudio;
extern crate rand;
extern crate termion;
extern crate time;

mod chip8;
mod cpu;
mod display;
mod keyboard;
mod memory_bus;
mod opcodes;
mod sound;
mod timer;
mod window;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::{Arg, App};

fn main() {

    // init logging
    let mut logger = env_logger::LogBuilder::new();
    let log_filters = env::var("RUST_LOG").unwrap_or("info".into());
    logger.parse(&log_filters);
    logger.init()
          .expect("failed to initialize logging");

    debug!("///////////////////////////");
    debug!("/ Running CHIP-8 emulator /");
    debug!("///////////////////////////");

    // parse command line args
    let args = App::new("CHIP EMUL8")
        .version("0.1")
        .author("Jeff Belgum <jeffbelgum@gmail.com>")
        .arg(Arg::with_name("ROM_FILE")
             .help("Sets the rom file to load")
             .required(true)
             .index(1))
        .arg(Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("starts the emulator up in debug mode to step through instructions one at a time"))
        .arg(Arg::with_name("disassemble")
             .long("dis")
             .help("prints disassembled rom"))
        .arg(Arg::with_name("cycles")
             .short("c")
             .long("cycles")
             .takes_value(true)
             .help("number for cycles to execute before exiting the emulator"))
        .get_matches();

    // load rom
    let path = args.value_of("ROM_FILE").unwrap();
    let bin_file = load_bin(path);

    if args.is_present("disassemble") {
        // print disassembled code
        chip8::Chip8::disassemble(&bin_file);
    } else {
        let step = args.is_present("debug");
        let cycles: Option<u64> = args.value_of("cycles")
                                      .map(|c| c.parse().ok())
                                      .unwrap_or(None);
        // create and run chip-8 emulator
        chip8::Chip8::run(&bin_file, step, cycles);
    }
}

fn load_bin<P>(path: P) -> Vec<u8>
    where P: AsRef<Path> + fmt::Debug
{
    let mut file = File::open(&path).expect("Could not open ROM file");
    let mut buf = vec![];
    file.read_to_end(&mut buf).expect("Could not read ROM file contents");
    debug!("Read {} bytes from bin file {:?}", buf.len(), path);
    buf
}
