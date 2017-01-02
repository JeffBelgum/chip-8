use std::io::{Write, stdout, Stdout};

use termion;
use termion::cursor;
use termion::raw::{IntoRawMode,RawTerminal};

use memory_bus::MemoryBus;

// TODO: this could be a trait we can use with terminal, graphical interface, etc
pub struct Display {
    out: RawTerminal<Stdout>,
}

impl Display {
    pub fn new() -> Display {
        let mut display = Display {
            out: stdout().into_raw_mode()
                         .expect("failed to put terminal into raw mode."),
        };
        write!(display.out, "{}", cursor::Hide);
        display.clear();
        display

    }

    /// clear the entire display
    pub fn clear(&mut self) {
        writeln!(self.out, "{}", termion::clear::All);
    }
    /// Draws a sprite at coordinate (x, y) that has a width of 8 pixels 
    /// and a height of n pixels. Each row of 8 pixels is read as 
    /// bit-coded starting from memory location i; i value doesn’t change 
    /// after the execution of this instruction. 
    ///
    /// returns true if any screen pixels are flipped from set to unset when 
    /// the sprite is drawn, and false if that doesn’t happen.
    pub fn draw(&mut self, x: u16, y: u16, n: usize, mem_bus: &mut MemoryBus, i: usize) -> bool {
        debug!("drawing 8x{} block at ({},{}) with data 0b{:08b}{:08b}{:08b}{:08b}",
               n, x, y, 
               mem_bus.read_word(i),
               mem_bus.read_word(i+1),
               mem_bus.read_word(i+2),
               mem_bus.read_word(i+3)
        );
        let block = "█";
        let blocks = ["▀", "▄"];
        // termion coordinate system is one-based
        let x = x + 1;
        let y = y + 1;

        for j in 0..n {
            let y_j = y + j as u16;
            write!(self.out, "{}", cursor::Goto(x, y_j));
            let word = mem_bus.read_word(i+j);
            debug!("drawing row 0b{:08b}", word);
            for shift in 0..8 {
                let shift = 7 - shift;
                let pixel = (word >> shift) & 1;
                debug!("drawing pixel {}", pixel);
                if pixel == 1 {
                    write!(self.out, "{}", block);
                } else {
                    write!(self.out, " ");
                }
            }
        }
        // TODO: check and flip if needed
        false
    }

}
