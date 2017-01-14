use std::sync::{Arc, Mutex};
use time;

use log::LogLevel;

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
        debug!("drawing 8x{} block at ({},{}) with data 0b{:08b}{:08b}{:08b}{:08b}",
               n, x, y, 
               mem_bus.read_word(i),
               mem_bus.read_word(i+1),
               mem_bus.read_word(i+2),
               mem_bus.read_word(i+3)
        );
        let prep_start = time::get_time();
        let mut unset_flag = false;
        let mut temp_grid = vec![2; self.grid.len()];
        for y_offset in 0..n {
            let word = mem_bus.read_word(i + y_offset);
            for x_offset in 0..8 {
                let pixel = (word >> (7 - x_offset)) & 1;
                let (_x, _y) = (x as usize + x_offset, y as usize + y_offset);
                // ignore pixels that run off the edge of the row
                if _x >= WIDTH || _y >= HEIGHT {
                    continue;
                }
                let idx = _y * WIDTH + _x;
                if self.grid[idx] == 1 && pixel == 1 {
                    unset_flag = true;
                }
                self.grid[idx] ^= pixel;
                temp_grid[idx] = pixel;
            }
        }
        if log_enabled!(LogLevel::Debug) {
            for row in temp_grid.chunks(WIDTH) {
                let mut row_str: String = "".into();
                for pixel in row.iter() {
                    row_str.push(if *pixel == 1 {'1'} else if *pixel == 0 {'0'} else { '_' });
                }
                debug!("{}", row_str);
            }
            debug!("");
            for row in self.grid.chunks(WIDTH) {
                let mut row_str: String = "".into();
                for pixel in row.iter() {
                    row_str.push(if *pixel == 1 {'1'} else {'0'});
                }
                debug!("{}", row_str);
            }
        }
        let prep_end = time::get_time();
        self.window.lock()
                   .expect("failed to aquire lock")
                   .draw(&self.grid);
        let draw_end = time::get_time();

        let prep_time = (prep_end - prep_start).num_nanoseconds().unwrap();
        let draw_time = (draw_end - prep_end).num_nanoseconds().unwrap();
        info!("prep time: {}, draw time: {}", prep_time / 1000, draw_time / 1000);

        unset_flag
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        // write!(self.out, "{}", cursor::Show);
    }
}
