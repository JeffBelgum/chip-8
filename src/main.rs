#![allow(dead_code,unused_variables)]

extern crate env_logger;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate portaudio;
extern crate rand;
extern crate termion;

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

fn main() {

    // init logging
    // let mut logger = env_logger::LogBuilder::new();
    // logger.parse("info");
    // logger.init().unwrap();
    env_logger::init().unwrap();
    debug!("");
    debug!("");
    debug!("///////////////////////////");
    debug!("/ Running CHIP-8 emulator /");
    debug!("///////////////////////////");
    debug!("");

    // load rom
    let path = env::args().nth(1).unwrap();
    let bin_file = load_bin(path);

    // create and run chip-8 emulator
    // chip8::Chip8::run(&bin_file);

    // print disassembled code
    chip8::Chip8::disassemble(&bin_file);
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
