use std::fmt;

// 2 words per opcode
pub const OP_SIZE: usize = 2;

#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    Eof,                                 // 0x0A00
    DrawClr,                             // 0x00E0
    Return,                              // 0x00EE
    JpConst { nnn: usize },              // 0x1NNN
    Call { nnn: usize },                 // 0x2NNN
    SkpEqConst { x: usize, nn: u8 },     // 0x3XNN
    SkpNeConst { x: usize, nn: u8 },     // 0x4XNN
    SkpEqReg { x: usize, y: usize },     // 0x5XY0
    SetConst { x: usize, nn: u8 },       // 0x6XNN
    AddConst { x: usize, nn: u8 },       // 0x7XNN
    SetReg { x: usize, y: usize },       // 0x8XY0
    SetRegBor { x: usize, y: usize },    // 0x8XY1
    SetRegBand { x: usize, y: usize },   // 0x8XY2
    SetRegBxor { x: usize, y: usize },   // 0x8XY3
    SetRegAdd { x: usize, y: usize },    // 0x8XY4
    SetRegSub { x: usize, y: usize },    // 0x8XY5
    SetShr1 { x: usize },                // 0x8XY6
    SetRegRevSub { x: usize, y: usize }, // 0x8XY7
    SetShl1 { x: usize },                // 0x8XYE
    JpRegNe { x: usize, y: usize },      // 0x9XY0
    SetI { nnn: u16 },                   // 0xANNN
    JpOffset { nnn: usize },             // 0xBNNN
    SetRand { x: usize, nn: u8 },        // 0xCXNN
    Draw { x: usize, y: usize, n: u8 },  // 0xDXYN
    SkpKeyEq { x: usize },               // 0xEX9E
    SkpKeyNe { x: usize },               // 0xEXA1
    SetRegDelay { x: usize },            // 0xFX07
    SetKey { x: usize },                 // 0xFX0A
    SetDelay { x: usize },               // 0xFX15
    SetSound { x: usize },               // 0xFX18
    SetIRegAdd { x: usize },             // 0xFX1E
    SetISprite { x: usize },             // 0xFX29
    SetBCD { x: usize },                 // 0xFX33
    DumpReg { x: usize },                // 0xFX55
    LoadReg { x: usize },                // 0xFX65
    Unknown(u16)                         // unrecognized opcode
}

impl From<u16> for OpCode {
    fn from(instr: u16) -> Self {
        use self::OpCode::*;

        let op =  ((instr & 0xF000) >> 12) as u16;
        let x =   ((instr & 0x0F00) >>  8) as usize;
        let y =   ((instr & 0x00F0) >>  4) as usize;
        let n =   ((instr & 0x000F)      ) as u8;
        let nn =  ((instr & 0x00FF)      ) as u8;
        let nnn = ((instr & 0x0FFF)      ) as usize;

        match op {
            0x0 => {
                match instr {
                    0x0A00 => Eof,
                    0x00E0 => DrawClr,
                    0x00EE => Return,
                    _ => Unknown(instr),
                }
            }
            0x1 => JpConst { nnn: nnn },
            0x2 => Call { nnn: nnn },
            0x3 => SkpEqConst { x: x, nn: nn },
            0x4 => SkpNeConst { x: x, nn: nn },
            0x5 => SkpEqReg { x: x, y: y },
            0x6 => SetConst { x: x, nn: nn },
            0x7 => AddConst { x: x, nn: nn },
            0x8 => {
                let op_2 = n;
                match op_2 {
                    0x0 => SetReg { x: x, y: y },
                    0x1 => SetRegBor { x: x, y: y },
                    0x2 => SetRegBand { x: x, y: y },
                    0x3 => SetRegBxor { x: x, y: y },
                    0x4 => SetRegAdd { x: x, y: y },
                    0x5 => SetRegSub { x: x, y: y },
                    0x6 => SetShr1 { x: x },
                    0x7 => SetRegRevSub { x: x, y: y },
                    0xE => SetShl1 { x: x },
                    _ => Unknown(instr),
                }
            }
            0x9 => JpRegNe { x: x, y: y },
            0xA => SetI { nnn: nnn as u16 },
            0xB => JpOffset { nnn: nnn},
            0xC => SetRand { x: x, nn: nn },
            0xD => Draw { x: x, y: y, n: n },
            0xE => {
                let op_2 = nn;
                match op_2 {
                    0x9E => SkpKeyEq { x: x },
                    0xA1 => SkpKeyNe { x: x },
                    _ => Unknown(instr),
                }
            }
            0xF => {
                let op_2 = nn;
                match op_2 {
                    0x07 => SetRegDelay { x: x },
                    0x0A => SetKey { x: x },
                    0x15 => SetDelay { x: x },
                    0x18 => SetSound { x: x },
                    0x1E => SetIRegAdd { x: x },
                    0x29 => SetISprite { x: x },
                    0x33 => SetBCD { x: x },
                    0x55 => DumpReg { x: x },
                    0x65 => LoadReg { x: x },
                    _ => Unknown(instr),
                }
            }
            _ => Unknown(instr),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCode::Eof =>               write!(f, "EOF"),
            OpCode::DrawClr =>           write!(f, "draw clear"),
            OpCode::Return =>            write!(f, "return"),
            OpCode::JpConst{nnn} =>      write!(f, "jump 0x{:03X}", nnn),
            OpCode::Call{nnn} =>         write!(f, "call 0x{:03X}", nnn),
            OpCode::SkpEqConst{x,nn} =>  write!(f, "skip V{:X}==0x{:02X}", x, nn),
            OpCode::SkpNeConst{x,nn} =>  write!(f, "skip V{:X}!=0x{:02X}", x, nn),
            OpCode::SkpEqReg{x,y} =>     write!(f, "skip V{:X}==V{:X}", x, y),
            OpCode::SetConst{x,nn} =>    write!(f, "set  V{:X}=0x{:02X}", x, nn),
            OpCode::AddConst{x,nn} =>    write!(f, "set  V{:X}+=0x{:02X}", x, nn),
            OpCode::SetReg{x,y} =>       write!(f, "set  V{:X}=V{:X}", x, y),
            OpCode::SetRegBor{x,y} =>    write!(f, "set  V{:X}|=V{:X}", x, y),
            OpCode::SetRegBand{x,y} =>   write!(f, "set  V{:X}&=V{:X}", x, y),
            OpCode::SetRegBxor{x,y} =>   write!(f, "set  V{:X}^=V{:X}", x, y),
            OpCode::SetRegAdd{x,y} =>    write!(f, "set  V{:X}+=V{:X}", x, y),
            OpCode::SetRegSub{x,y} =>    write!(f, "set  V{:X}-=V{:X}", x, y),
            OpCode::SetShr1{x} =>        write!(f, "set  V{:X}>>=1", x),
            OpCode::SetRegRevSub{x,y} => write!(f, "set  V{:X}=V{:X} - V{:X}", x, y, x),
            OpCode::SetShl1{x} =>        write!(f, "set  V{:X}<<=1", x),
            OpCode::JpRegNe{x,y} =>      write!(f, "jump V{:X}!=V{:X}", x, y),
            OpCode::SetI{nnn} =>         write!(f, "set  I=0x{:03X}", nnn),
            OpCode::JpOffset{nnn} =>     write!(f, "jump V0 + 0x{:03X}", nnn),
            OpCode::SetRand{x,nn} =>     write!(f, "set  V{:X}=rand() & {:02X}", x, nn),
            OpCode::Draw{x,y,n} =>       write!(f, "draw V{:X}, V{:X}, {} ", x, y, n),
            OpCode::SkpKeyEq{x} =>       write!(f, "skip V{:X} == key()", x),
            OpCode::SkpKeyNe{x} =>       write!(f, "skip V{:X} != key()", x),
            OpCode::SetRegDelay{x} =>    write!(f, "set  V{:X}=delay_timer", x),
            OpCode::SetKey{x} =>         write!(f, "set  V{:X}=get_key()", x),
            OpCode::SetDelay{x} =>       write!(f, "set  delay_timer=V{:X}", x),
            OpCode::SetSound{x} =>       write!(f, "set  sound_timer=V{:X}", x),
            OpCode::SetIRegAdd{x} =>     write!(f, "set  I=I+V{:X}", x),
            OpCode::SetISprite{x} =>     write!(f, "set  I=sprite_addr[V{:X}]", x),
            OpCode::SetBCD{x} =>         write!(f, "set  bcd V{:X}", x),
            OpCode::DumpReg{x} =>        write!(f, "dump V0..V{:X} to *I", x),
            OpCode::LoadReg{x} =>        write!(f, "load V0..V{:X} from *I", x),
            OpCode::Unknown(instr) =>    write!(f, "unrecognized instruction"),
        }
    }
}
