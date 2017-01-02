const MEMORY_SIZE: usize = 4 * 1024;
const ROM_START: usize = 0x200;

pub struct MemoryBus {
    mem: Box<[u8]>
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            mem: vec![0; MEMORY_SIZE].into_boxed_slice(),
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
        let slice = &mut self.mem[start..end];
        slice.copy_from_slice(rom);
    }
}
