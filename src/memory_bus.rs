const MEMORY_SIZE: usize = 4 * 1024;
pub const ROM_START: usize = 0x200;

const FONT_START: usize = 0x050;
const FONT_END: usize = 0x0A0;
const FONT_SPRITE_STRIDE: usize = 5;
const FONT_SPRITES: [u8; 80] =
[ 
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct MemoryBus {
    mem: Box<[u8]>
}

impl MemoryBus {
    pub fn font_sprite_address(hex_char: u8) -> Option<usize> {
        match hex_char {
            c @ 0x0...0xF => Some(FONT_START + (c as usize) * FONT_SPRITE_STRIDE),
            _ => None,
        }
    }

    pub fn new() -> MemoryBus {
        let mut mem = vec![0; MEMORY_SIZE].into_boxed_slice();
        // copy font sprite data into memory
        {
            let dst = &mut mem[FONT_START..(FONT_END)];
            dst.copy_from_slice(&FONT_SPRITES);
        }
        MemoryBus {
            mem: mem,
        }
    }

    pub fn read_word(&self, addr: usize) -> u8 {
        *self.mem.get(addr).expect("attempting to read from invalid memory address")
    }

    pub fn write_word(&mut self, addr: usize, value: u8) {
        match self.mem.get_mut(addr) {
            Some(word) => *word = value,
            None => panic!("attempting to write invalid memory address"),
        }
    }

    pub fn write_words(&mut self, addr: usize, src: &[u8]) {
        let start = addr;
        let end = start + src.len();
        if end > self.mem.len() {
            panic!("attempting to write invalid memory address");
        }
        let dst = &mut self.mem[start..end];
        dst.copy_from_slice(src);
    }

    pub fn read_words(&self, addr: usize, len: usize) -> &[u8] {
        let start = addr;
        let end = start + len;
        if end > self.mem.len() {
            panic!("attempting to read invalid memory address");
        }
        &self.mem[start..end]
    }

    pub fn read_instruction(&self, addr: usize) -> u16 {
        // debug!("{:X}", addr);
        let word_1 = *self.mem.get(addr)
                              .expect("attempting to read from invalid memory address");
        if word_1 == 0x0A {
            0x0A00
        } else {
            let word_2 = *self.mem.get(addr + 1)
                                  .expect("attempting to read from invalid memory address");
            // opcodes are big-endian
            ((word_1 as u16) << 8) | (word_2 as u16)
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let start = ROM_START;
        let end = start + rom.len();
        let dst = &mut self.mem[start..end];
        dst.copy_from_slice(rom);
    }
}
