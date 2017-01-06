use std::collections::HashMap;

use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{ElementState, Event};
use glium::{
    DisplayBuild,
    VertexBuffer,
    IndexBuffer,
    Program,
    Surface,
    glutin,
    index,
    texture,
    uniforms,
};

use keyboard::Keyboard;

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

pub struct Window {
    display: GlutinFacade,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
    key_states: HashMap<u8, ElementState>,
}

impl Window {
    pub fn new(grid_width: usize, grid_height: usize) -> Window {
        let display = glutin::WindowBuilder::new()
            .with_dimensions(640, 320)
            .with_title("CHIP-8")
            .build_glium()
            .expect("failed to build glutin window");
        let vertex_buffer = VertexBuffer::new(&display, &SLAB).expect("VB");
        let index_buffer = IndexBuffer::new(&display, index::PrimitiveType::TriangleStrip,
                                 &INDICES).expect("failed to create index buffer");
        let program = Program::from_source(&display, VERT_SHADER, FRAG_SHADER, None).expect("failed to create program");

        // init key states for window
        let key_states = (0x0..0x10).map(|code| (code, ElementState::Released)).collect();

        Window {
            display: display,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            key_states: key_states,
        }
    }

    pub fn poll_events(&mut self) {
        for ev in self.display.poll_events() {
            match ev {
                Event::KeyboardInput(state, _, Some(key)) => {
                    if let Some(hex_code) = Keyboard::remap_key(key) {
                        self.key_states.insert(hex_code, state);
                    }
                }
                _ => (),
            }
        }
    }

    pub fn get_key_states(&mut self) -> &HashMap<u8, ElementState> {
        self.poll_events();
        &self.key_states
    }

    pub fn get_key(&mut self) -> u8 {
        for ev in self.display.wait_events() {
            match ev {
                Event::KeyboardInput(state, _, Some(key)) => {
                    if let Some(hex_code) = Keyboard::remap_key(key) {
                        debug!("{:?} -> {:X}", key, hex_code);
                        self.key_states.insert(hex_code, state);
                        if let ElementState::Pressed = state {
                            return hex_code;
                        }
                    } else {
                        debug!("{:?} -> None", key);
                    }
                }
                _ => (),
            }
        }
        unreachable!()
    }

    pub fn draw(&mut self, grid: &[u8]) {
        let mut image_buffer = Vec::with_capacity(320);
        for y in 0..320 {
            let y = 319 - y;
            let mut row = Vec::with_capacity(640);
            for x in 0..640 {
                let grid_idx = (y / 10) * 64 + (x / 10);
                if grid_idx == 2048 {
                    error!("uh oh index");
                }
                
                let pixel = if grid[grid_idx] == 1 {
                    (1.0, 1.0, 1.0, 1.0)
                } else {
                    (0.0, 0.0, 0.0, 1.0)
                };
                // let pixel = (y as f32 / 319.0, x as f32 / 639.0, 0.0, 1.0);
                row.push(pixel);
            }
            image_buffer.push(row);
        }
        let tex = texture::SrgbTexture2d::with_mipmaps(&self.display, image_buffer, texture::MipmapsOption::NoMipmap)
            .expect("tex2d");
        let uniforms = uniform! {
            tex: uniforms::Sampler::new(&tex).magnify_filter(uniforms::MagnifySamplerFilter::Nearest),
        };

        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &Default::default()).expect("frame draw");
        frame.finish().expect("finishing frame");
        self.poll_events();
    }
}
