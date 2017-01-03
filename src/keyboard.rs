use std::io::{Read, stdin, Stdin};

use termion::event::Key;
use termion::input::{Keys, TermRead};

pub struct Keyboard {
    _in: Keys<Stdin>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            _in: stdin().keys(),
        }
    }

    /// block the next valid keypress and return it as a hex value 0x0 - 0xF
    pub fn get_key(&mut self) -> u8 {
        loop {
            if let Some(key) = self._in.next() {
                if let Ok(key) = key {
                    if let Some(hex) = Keyboard::remap_key(key) {
                        debug!("{:?} -> {:X}", key, hex);
                        return hex;
                    } else {
                        debug!("{:?} -> None", key);
                    }
                }
            }
        }
        0x0
    }

    pub fn remap_key(key: Key) -> Option<u8> {
        match key {
            Key::Char('v') => Some(0x0),
            Key::Char('q') => Some(0x1),
            Key::Char('w') => Some(0x2),
            Key::Char('e') => Some(0x3),
            Key::Char('a') => Some(0x4),
            Key::Char('s') => Some(0x5),
            Key::Char('d') => Some(0x6),
            Key::Char('z') => Some(0x7),
            Key::Char('x') => Some(0x8),
            Key::Char('c') => Some(0x9),
            Key::Char('1') => Some(0xA),
            Key::Char('2') => Some(0xB),
            Key::Char('3') => Some(0xC),
            Key::Char('4') => Some(0xD),
            Key::Char('r') => Some(0xE),
            Key::Char('f') => Some(0xF),
            Key::Ctrl('c') => {
                panic!("ctrl-c");
            }
            _ => None
        }
    }
}
