use std::io::{Write, stdout, Stdout};

use termion;
use termion::cursor;
use termion::raw::{IntoRawMode,RawTerminal};

use memory_bus::MemoryBus;

const FULL_BLOCK: &'static str = "█";
const TOP_BLOCK: &'static str = "▀";
const BOTTOM_BLOCK: &'static str = "▄";

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
        // termion coordinate system is one-based
        let x = x + 1;
        let y = y + 1;

        // sprites are drawn 2 rows at a time in order to use unicode half-blocks, "▄",
        // for each row.
        let mut j = 0;
        while j < n {
            // position cursor
            let y_j = (y + j as u16) / 2;
            write!(self.out, "{}", cursor::Goto(x, y_j));

            // read words
            let top_word = mem_bus.read_word(i + j);
            let bottom_word = if j != n - 1 {
                mem_bus.read_word(i + j + 1)
            } else {
                0
            };

            // loop through bits in both words simultaniously
            for shift in 0..8 {
                let shift = 7 - shift;

                let top_pixel = (top_word >> shift) & 1;
                let bottom_pixel = (bottom_word >> shift) & 1;
                if top_pixel == 1 && bottom_pixel == 1 {
                    write!(self.out, "{}", FULL_BLOCK);
                } else if top_pixel == 1{
                    write!(self.out, "{}", TOP_BLOCK);
                } else if bottom_pixel == 1 {
                    write!(self.out, "{}", BOTTOM_BLOCK);
                } else {
                    write!(self.out, " ");
                }
            }
            j += 2;
        }
        // TODO: check and flip if needed
        false
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        write!(self.out, "{}", cursor::Show);
    }
}
