use std::sync::{Arc, Mutex};

use memory_bus::MemoryBus;
use window::Window;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    grid: [u8; WIDTH * HEIGHT],
    window: Arc<Mutex<Window>>,
}

impl Display {
    pub fn new(window: Arc<Mutex<Window>>) -> Display {
        let mut display = Display {
            grid: [0; WIDTH * HEIGHT],
            window: window,
        };
        display.clear();
        display
    }

    /// clear the entire display
    pub fn clear(&mut self) {
        for x in &mut self.grid[..] {
            *x = 0
        }
        self.window.lock()
                   .expect("failed to aquire lock")
                   .draw(&self.grid);
    }
    /// Draws a sprite at coordinate (x, y) that has a width of 8 pixels 
    /// and a height of n pixels. Each row of 8 pixels is read as 
    /// bit-coded starting from memory location i; i value doesn’t change 
    /// after the execution of this instruction. 
    ///
    /// returns true if any screen pixels are flipped from set to unset when 
    /// the sprite is drawn, and false if that doesn’t happen.
    pub fn draw(&mut self, x: u16, y: u16, n: usize, mem_bus: &mut MemoryBus, i: usize) -> bool {
        info!("drawing 8x{} block at ({},{}) with data 0b{:08b}{:08b}{:08b}{:08b}",
               n, x, y, 
               mem_bus.read_word(i),
               mem_bus.read_word(i+1),
               mem_bus.read_word(i+2),
               mem_bus.read_word(i+3)
        );
        let mut unset_flag = false;
        for y_offset in 0..n {
            let word = mem_bus.read_word(i + y_offset);
            for x_offset in 0..8 {
                let pixel = (word >> (7 - x_offset)) & 1;
                let idx = (y as usize + y_offset) * WIDTH + (x as usize + x_offset);
                if idx >= 2048 {
                    error!("uh oh mod idx x: {} x_o: {} y: {} y_o: {}", x, x_offset, y, y_offset);
                    continue;
                }
                if self.grid[idx] == 1 && pixel == 0 {
                    unset_flag = true;
                }
                self.grid[idx] = pixel;
            }
        }
        self.window.lock()
                   .expect("failed to aquire lock")
                   .draw(&self.grid);

        // for now do full grid update -- it's simpler
        // write!(self.out, "{}", cursor::Goto(1, 1));
        // for row in self.grid.chunks(WIDTH) {
        //     let row_str = row.iter()
        //                      .map(|pixel| if *pixel == 1 { '█' } else if *pixel == 0 { ' ' } else { 'X' })
        //                      .collect::<String>();
        //     debug!("{}", row_str);
        //     writeln!(self.out, "{}", row_str);
        // }

        // // termion coordinate system is one-based
        // let x = x + 1;
        // let y = y + 1;

        // // sprites are drawn 2 rows at a time in order to use unicode half-blocks, "▄",
        // // for each row.
        // let mut j = 0;
        // while j < n {
        //     // position cursor
        //     let y_j = (y + j as u16) / 2;
        //     write!(self.out, "{}", cursor::Goto(x, y_j));

        //     // read words
        //     let top_word = mem_bus.read_word(i + j);
        //     let bottom_word = if j != n - 1 {
        //         mem_bus.read_word(i + j + 1)
        //     } else {
        //         0
        //     };

        //     // loop through bits in both words simultaniously
        //     for shift in 0..8 {
        //         let shift = 7 - shift;

        //         let top_pixel = (top_word >> shift) & 1;
        //         let bottom_pixel = (bottom_word >> shift) & 1;
        //         if top_pixel == 1 && bottom_pixel == 1 {
        //             write!(self.out, "{}", FULL_BLOCK);
        //         } else if top_pixel == 1{
        //             write!(self.out, "{}", TOP_BLOCK);
        //         } else if bottom_pixel == 1 {
        //             write!(self.out, "{}", BOTTOM_BLOCK);
        //         } else {
        //             write!(self.out, " ");
        //         }
        //     }
        //     j += 2;
        // }
        unset_flag
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        // write!(self.out, "{}", cursor::Show);
    }
}
