use std::process;
use rand::random;

use display::Display;
use memory_bus::MemoryBus;

// register names
const V0: usize = 0x0;
const V1: usize = 0x1;
const V2: usize = 0x2;
const V3: usize = 0x3;
const V4: usize = 0x4;
const V5: usize = 0x5;
const V6: usize = 0x6;
const V7: usize = 0x7;
const V8: usize = 0x8;
const V9: usize = 0x9;
const VA: usize = 0xA;
const VB: usize = 0xB;
const VC: usize = 0xC;
const VD: usize = 0xD;
const VE: usize = 0xE;
const VF: usize = 0xF;

// 2 words per opcode
const OP_SIZE: usize = 2;

// 16 is a common stack size in modern chip-8 implementations
const STACK_SIZE: usize = 16;

pub struct Cpu {
    // Registers
    // program counter
    reg_pc: usize, // TODO: what is the size of this?
    // CHIP-8 has 16 8-bit data registers named from V0 to VF
    reg_vx: [u8; 16],
    // The address register, I, is 16 bits and is used with several
    // opcodes that involve memory operations.
    reg_i: u16,

    // The stack is used to store return addresses when subroutines are called.
    // We use 16 levels of nesting.
    stack: Vec<usize>,
    
    // counts the number of instructions executed
    counter: u64,
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_pc: 0x200, // pc starts here
            reg_vx: [0; 16],
            reg_i: 0,
            stack: Vec::with_capacity(STACK_SIZE),
            counter: 0,
        }
    }

    pub fn instruction_count(&self) -> u64 {
        self.counter
    }

    pub fn execute_instruction(&mut self, memory_bus: &mut MemoryBus, display: &mut Display) {
        let instr = memory_bus.read_instruction(self.reg_pc);
        let pc = self.reg_pc;
        self.counter += 1;
        self.reg_pc += OP_SIZE;
        let nibble_1 = (instr & 0xF000) >> 12;
        match nibble_1 {
            0x0 => {
                match instr {
                    0x0A00 => {
                        debug!("End of ROM");
                        process::exit(0);
                    }
                    0x00E0 => {
                        panic!("clear display -- unimplemented");
                    }
                    0x00EE => {
                        self.reg_pc = self.stack
                                          .pop()
                                          .expect("not in subroutine; cannot return");
                    }
                    _ => panic!("Invalid instruction: {:X}", instr)
                }
            }
            0x1 => {
                self.reg_pc = (0xFFF & instr) as usize;
            }
            0x2 => {
                if self.stack.len() >= STACK_SIZE {
                    panic!("subroutine nesting limit reached");
                }
                self.stack.push(self.reg_pc);
                self.reg_pc = (instr & 0xFFF) as usize;
            }
            0x3 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let nn = (instr & 0xFF) as u8;
                if self.reg_vx[x] == nn {
                    self.reg_pc += OP_SIZE;
                }
            }
            0x4 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let nn = (instr & 0xFF) as u8;
                if self.reg_vx[x] != nn{
                    self.reg_pc += OP_SIZE;
                }
            }
            0x5 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let y = ((instr &  0xF0) >> 4) as usize;
                if self.reg_vx[x] == self.reg_vx[y] {
                    self.reg_pc += OP_SIZE;
                }
            }
            0x6 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let nn = (instr & 0xFF) as u8;
                self.reg_vx[x] = nn;
            }
            0x7 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let nn = (instr & 0xFF) as u8;
                self.reg_vx[x] = self.reg_vx[x].wrapping_add(nn);
            }
            0x8 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let y = ((instr &  0xF0) >> 4) as usize;
                let op = (instr &   0xF) >> 0;

                match op {
                    0x0 => {
                        self.reg_vx[x] = self.reg_vx[y];
                    }
                    0x1 => {
                        self.reg_vx[x] = self.reg_vx[x] | self.reg_vx[y];
                    }
                    0x2 => {
                        self.reg_vx[x] = self.reg_vx[x] & self.reg_vx[y];
                    }
                    0x3 => {
                        self.reg_vx[x] = self.reg_vx[x] ^ self.reg_vx[y];
                    }
                    0x4 => {
                        let result = self.reg_vx[x] as u16 + self.reg_vx[y] as u16;
                        self.reg_vx[VF] = if result > 0xFF { 1 } else { 0 };
                        self.reg_vx[x] = result as u8;
                    }
                    0x5 => {
                        let result = self.reg_vx[x] as i16 - self.reg_vx[y] as i16;
                        self.reg_vx[VF] = if result < 0 { 1 } else { 0 };
                        self.reg_vx[x] = result as u8;
                    }
                    0x6 => {
                        self.reg_vx[VF] = self.reg_vx[x] & 1;
                        self.reg_vx[x] >>= 1;
                    }
                    0x7 => {
                        let result = self.reg_vx[y] as i16 - self.reg_vx[x] as i16;
                        self.reg_vx[VF] = if result < 0 { 1 } else { 0 };
                        self.reg_vx[x] = result as u8;
                    }
                    0xE => {
                        self.reg_vx[VF] = self.reg_vx[x] & 0b1000_0000;
                        self.reg_vx[x] <<= 1;
                    }
                    _ => panic!("Invalid instruction: {:X}", instr)
                }
            }
            0x9 => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let y = ((instr &  0xF0) >> 4) as usize;
                if self.reg_vx[x] != self.reg_vx[y] {
                    self.reg_pc += OP_SIZE;
                }
            }
            0xA => {
                self.reg_i = 0xFFF & instr;
            }
            0xB => {
                self.reg_pc = self.reg_vx[V0] as usize + (0xFFF & instr) as usize;
            }
            0xC => {
                // debug!("rnd");
                let x = ((instr & 0x0F00) >> 8) as usize;
                let nn = (instr & 0xFF) as u8;
                self.reg_vx[x] = random::<u8>() & nn;
            }
            0xD => {
                let x = ((instr & 0xF00) >> 8) as usize;
                let y = ((instr &  0xF0) >> 4) as usize;
                let n = ((instr &   0xF) >> 0) as usize;

                let vx = self.reg_vx[x] as u16;
                let vy = self.reg_vx[y] as u16;
                let i = self.reg_i as usize;

                let flipped_unset = display.draw(vx, vy, n, memory_bus, i);
                self.reg_vx[VF] = if flipped_unset { 1 } else { 0 };
            }
            _ => panic!("Unimplemented instruction: {:X}", instr)
        }
        debug!("{:04} 0x{:04X} {:04X}: V0=0x{:02X} V1=0x{:02X} V2=0x{:02X} I=0x{:03X}", 
               self.counter,
               pc,
               instr,
               self.reg_vx[V0],
               self.reg_vx[V1],
               self.reg_vx[V2],
               self.reg_i,
               );

        // debug!("{:?}", self.reg_vx);
    }
}
