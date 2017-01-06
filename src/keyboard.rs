use std::sync::{Arc, Mutex};
use glium::glutin::{ElementState, VirtualKeyCode};

use window::Window;

pub struct Keyboard {
    window: Arc<Mutex<Window>>,
}

impl Keyboard {
    pub fn new(window: Arc<Mutex<Window>>) -> Keyboard {
        Keyboard {
            window: window,
        }
    }

    /// block the next valid keypress and return it as a hex value 0x0 - 0xF
    pub fn get_key(&mut self) -> u8 {
        let mut window = self.window.lock().expect("failed to aquire lock");
        window.get_key()
    }

    pub fn is_key_pressed(&mut self, hex_code: u8) -> bool {
        let mut window = self.window.lock().expect("failed to aquire lock");
        let key_states = window.get_key_states();
        debug!("looking for key {:X} in {:?}", hex_code, key_states);
        match key_states[&hex_code] {
            ElementState::Pressed => true,
            ElementState::Released => false,
        }
    }

    pub fn remap_code(hex_code: u8) -> VirtualKeyCode {
        use self::VirtualKeyCode::*;
        match hex_code {
            0x0 => V,
            0x1 => Q,
            0x2 => W,
            0x3 => E,
            0x4 => A,
            0x5 => S,
            0x6 => D,
            0x7 => Z,
            0x8 => X,
            0x9 => C,
            0xA => Key1,
            0xB => Key2,
            0xC => Key3,
            0xD => Key4,
            0xE => R,
            0xF => F,
            _ => panic!("Invalid key code {:X}", hex_code),
        }
    }

    pub fn remap_key(key: VirtualKeyCode) -> Option<u8> {
        use self::VirtualKeyCode::*;
        match key {
            V    => Some(0x0),
            Q    => Some(0x1),
            W    => Some(0x2),
            E    => Some(0x3),
            A    => Some(0x4),
            S    => Some(0x5),
            D    => Some(0x6),
            Z    => Some(0x7),
            X    => Some(0x8),
            C    => Some(0x9),
            Key1 => Some(0xA),
            Key2 => Some(0xB),
            Key3 => Some(0xC),
            Key4 => Some(0xD),
            R    => Some(0xE),
            F    => Some(0xF),
            Escape => {
                panic!("esc");
            }
            _ => None
        }
    }
}
