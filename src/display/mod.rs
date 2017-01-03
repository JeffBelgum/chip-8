use std::io::{Write, stdout, Stdout};

use glium::{
    DisplayBuild,
    VertexBuffer,
    IndexBuffer,
    Program,
    Surface,
    glutin,
    index,
    texture,
};

use memory_bus::MemoryBus;

const FULL_BLOCK: &'static str = "█";
const TOP_BLOCK: &'static str = "▀";
const BOTTOM_BLOCK: &'static str = "▄";
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const FRAG_SHADER: &'static str = r#"
#version 400

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
  color = texture(tex, v_tex_coords);
}
"#;

const VERT_SHADER: &'static str = r#"
#version 400

in vec4 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

void main() {
  v_tex_coords = tex_coords;
  gl_Position = position;
}
"#;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub const SLAB: [Vertex; 4] = [
    Vertex { position: [-1.0, -1.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [-1.0,  1.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 1.0, -1.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 1.0,  1.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },
];

pub const INDICES: [u16; 4] = [
    0, 1, 2, 3
];


// TODO: this could be a trait we can use with terminal, graphical interface, etc
pub struct Display {
    grid: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        let d = glutin::WindowBuilder::new()
            .with_dimensions(640, 320)
            .build_glium()
            .expect("failed to build glutin window");
        let v = VertexBuffer::new(&d, &SLAB).expect("VB");
        let i = IndexBuffer::new(&d, index::PrimitiveType::TriangleStrip,
                                 &INDICES).expect("failed to create index buffer");
        let p = Program::from_source(&d, VERT_SHADER, FRAG_SHADER, None).expect("failed to create program");
        let mut raw = Vec::with_capacity(320);
        for y in 0..320 {
            let mut row = Vec::with_capacity(640);
            for x in 0..640 {
                let pixel = if (((x / 10) & 1) == 1) ^ (((y / 10) & 1) == 1) {
                    (1.0, 1.0, 1.0, 1.0)
                } else {
                    (0.0, 0.0, 0.0, 1.0)
                };
                row.push(pixel);
            }
            raw.push(row);
        }
        let tex = texture::SrgbTexture2d::new(&d, raw).expect("tex2d");

        loop {
            let mut t = d.draw();
            t.clear_color(1.0, 0.0, 1.0, 1.0);
            t.draw(&v, &i, &p, &uniform!{tex: &tex}, &Default::default()).expect("t draw");
            t.finish().expect("t finish");
            for ev in d.poll_events() {
                debug!("{:?}", ev);
            }
        }

        // use ::std::{thread, time};
        // thread::sleep(time::Duration::from_millis(10000));

        let mut display = Display {
            grid: [0; WIDTH * HEIGHT],
        };
        // write!(display.out, "{}", cursor::Hide);
        display.clear();
        display

    }

    /// clear the entire display
    pub fn clear(&mut self) {
        // writeln!(self.out, "{}", termion::clear::All);
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
        let mut unset_flag = false;
        for y_offset in 0..n {
            let word = mem_bus.read_word(i + y_offset);
            for x_offset in 0..8 {
                let pixel = (word >> (7 - x_offset)) & 1;
                let idx = (y as usize + y_offset) * WIDTH + (x as usize + x_offset);
                debug!("{}", idx);
                if self.grid[idx] == 1 && pixel == 0 {
                    unset_flag = true;
                }
                self.grid[idx] = pixel;
            }
        }

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
