use rand::random;

use display::Display;
use keyboard::Keyboard;
use memory_bus::{MemoryBus, ROM_START};
use timer::Timer;
use opcodes::OP_SIZE;
use opcodes::OpCode::*;

// number of general purpose (VX) registers
const GP_REG_COUNT: usize = 16;

// general purpose register names
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

// 16 is a common stack size in modern chip-8 implementations
const STACK_SIZE: usize = 16;

pub struct Cpu {
    // Registers
    // program counter
    reg_pc: usize,
    // CHIP-8 has 16 8-bit data registers named from V0 to VF
    reg_vx: [u8; GP_REG_COUNT],
    // The address register, I, is 16 bits and is used with several
    // opcodes that involve memory operations.
    reg_i: u16,

    // The stack is used to store return addresses when subroutines are called.
    // We use 16 levels of nesting.
    stack: Vec<usize>,

    // counts the number of instructions executed
    counter: u64,

    // flag to let the system know whether to shut down
    // this is just for convenience and doesn't model the real CHIP-8 system
    exit: bool,
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_pc: ROM_START,
            reg_vx: [0; GP_REG_COUNT],
            reg_i: 0,
            stack: Vec::with_capacity(STACK_SIZE),
            counter: 0,
            exit: false,
        }
    }

    pub fn instruction_count(&self) -> u64 {
        self.counter
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn execute_instruction(&mut self,
                               memory_bus: &mut MemoryBus,
                               display: &mut Display,
                               keyboard: &mut Keyboard,
                               delay_timer: &mut Timer,
                               sound_timer: &mut Timer,
                               )
    {
        if self.exit {
            return;
        }

        let instr = memory_bus.read_instruction(self.reg_pc);
        let opcode = instr.into();


        self.counter += 1;

        // store current pc and bump before executing instruction
        // because some instructions modify the pc explicitly
        let pc = self.reg_pc;
        self.reg_pc += OP_SIZE;

        debug!("{:010} 0x{:03X} {:04X} {:?}", self.counter, pc, instr, opcode);

        // execute instruction logic
        match opcode {
            Eof => self.exit = true,
            DrawClr => display.clear(),
            Return => {
                self.reg_pc = self.stack
                   .pop()
                   .expect("not in subroutine; cannot return")
            }
            JpConst{nnn} => self.reg_pc = nnn,
            Call{nnn} => {
                if self.stack.len() >= STACK_SIZE {
                    panic!("subroutine nesting limit reached");
                }
                self.stack.push(self.reg_pc);
                self.reg_pc = nnn;
            }
            SkpEqConst{x,nn} => {
                if self.reg_vx[x] == nn {
                    self.reg_pc += OP_SIZE;
                }
            }
            SkpNeConst{x,nn} => {
                if self.reg_vx[x] != nn {
                    self.reg_pc += OP_SIZE;
                }
            }
            SkpEqReg{x,y} => {
                if self.reg_vx[x] == self.reg_vx[y] {
                    self.reg_pc += OP_SIZE;
                }
            }
            SetConst{x,nn} => self.reg_vx[x] = nn,
            AddConst{x,nn} => {
                self.reg_vx[x] = self.reg_vx[x].wrapping_add(nn);
            }
            SetReg{x,y} => self.reg_vx[x] = self.reg_vx[y],
            SetRegBor{x,y} => self.reg_vx[x] |= self.reg_vx[y],
            SetRegBand{x,y} => self.reg_vx[x] &= self.reg_vx[y],
            SetRegBxor{x,y} => self.reg_vx[x] ^= self.reg_vx[y],
            SetRegAdd{x,y} => {
                let result = self.reg_vx[x] as u16 + self.reg_vx[y] as u16;
                self.reg_vx[VF] = if result > 0xFF { 1 } else { 0 };
                self.reg_vx[x] = result as u8;
            }
            SetRegSub{x,y} => {
                let result = self.reg_vx[x] as i16 - self.reg_vx[y] as i16;
                self.reg_vx[VF] = if result < 0 { 1 } else { 0 };
                self.reg_vx[x] = result as u8;
            }
            SetShr1{x} => {
                self.reg_vx[VF] = self.reg_vx[x] & 1;
                self.reg_vx[x] >>= 1;
            }
            SetRegRevSub{x,y} => {
                let result = self.reg_vx[y] as i16 - self.reg_vx[x] as i16;
                self.reg_vx[VF] = if result < 0 { 1 } else { 0 };
                self.reg_vx[x] = result as u8;
            }
            SetShl1{x} => {
                self.reg_vx[VF] = self.reg_vx[x] & 0b1000_0000;
                self.reg_vx[x] <<= 1;
            }
            JpRegNe{x,y} => {
                if self.reg_vx[x] != self.reg_vx[y] {
                    self.reg_pc += OP_SIZE;
                }
            }
            SetI{nnn} => self.reg_i = nnn,
            JpOffset{nnn} => self.reg_pc = self.reg_vx[V0] as usize + nnn,
            SetRand{x,nn} => self.reg_vx[x] = random::<u8>() & nn,
            Draw{x,y,n} => {
                let vx = self.reg_vx[x] as u16;
                let vy = self.reg_vx[y] as u16;
                let i = self.reg_i as usize;
                let n = n as usize; // convert to usize here so we don't blow up size of enum

                let flipped_unset = display.draw(vx, vy, n, memory_bus, i);
                self.reg_vx[VF] = if flipped_unset { 1 } else { 0 };
            }
            SkpKeyEq{x} => {
                if keyboard.is_key_pressed(self.reg_vx[x]) {
                    self.reg_pc += OP_SIZE;
                }
            }
            SkpKeyNe{x} => {
                if !keyboard.is_key_pressed(self.reg_vx[x]) {
                    self.reg_pc += OP_SIZE;
                }
            }
            SetRegDelay{x} => self.reg_vx[x] = delay_timer.get_value(),
            SetKey{x} => self.reg_vx[x] = keyboard.get_key(),
            SetDelay{x} => delay_timer.set_value(self.reg_vx[x]),
            SetSound{x} => sound_timer.set_value(self.reg_vx[x]),
            SetIRegAdd{x} => {
                self.reg_i += self.reg_vx[x] as u16;
                self.reg_vx[VF] = if self.reg_i > 0xFFF { 1 } else { 0 };
            }
            SetISprite{x} => {
                let hex_char = self.reg_vx[x];
                self.reg_i = MemoryBus::font_sprite_address(hex_char)
                    .expect("Invalid hex char") as u16;
            }
            SetBCD{x} => {
                let i = self.reg_i as usize;
                let vx = self.reg_vx[x];

                let hundreds = vx / 100;
                let tens = (vx % 100) / 10;
                let ones = vx % 10;

                memory_bus.write_word(i, hundreds);
                memory_bus.write_word(i+1, tens);
                memory_bus.write_word(i+2, ones);
            }
            DumpReg{x} => {
                let len = x + 1;
                let dst_addr = self.reg_i as usize;
                let src = &self.reg_vx[0..len];
                memory_bus.write_words(dst_addr, src);
            }
            LoadReg{x} => {
                let len = x + 1;
                let src_addr = self.reg_i as usize;
                let src = memory_bus.read_words(src_addr, len);
                let dst = &mut self.reg_vx[0..len];
                dst.copy_from_slice(src);
            }
            Unknown(instr) => panic!("Invalid instruction {:X}", instr),
        };

    }

    pub fn print_cpu_state(&self) {
        debug!("{:04} PC=0x{:04X}: I=0x{:03X} V0=0x{:02X} V1=0x{:02X} V2=0x{:02X}",
               self.counter,
               self.reg_pc,
               self.reg_i,
               self.reg_vx[V0],
               self.reg_vx[V1],
               self.reg_vx[V2],
               );
    }
}
