use super::element_buffer::ElementBuffer;
use super::vertex_buffer::VertexBuffer;
use std::rc::Rc;

pub const VERTICES: [f32; 8] = [
    0.0, 0.0, // bottom left
    1.0, 0.0, // bottom right
    0.0, 1.0, // top left
    1.0, 1.0, // top right
];

const MIN_SIZE: usize = 32;

#[derive(Clone)]
pub struct Quad {
    pub ebo: Rc<ElementBuffer>,
    pub vbo: Rc<VertexBuffer<f32>>,
}

impl Quad {
    pub fn new(gl: Rc<glow::Context>) -> Quad {
        let mut ebo = ElementBuffer::new(gl.clone());
        let mut vbo = VertexBuffer::new(gl, 2);

        // prefill vertices
        for _ in 0..MIN_SIZE {
            vbo.buffer.extend_from_slice(&VERTICES);
        }
        vbo.update(glow::STATIC_DRAW);

        // prefill indices
        for i in 0..MIN_SIZE as u32 {
            ebo.buffer.extend_from_slice(&[
                i * 4,
                1 + i * 4,
                2 + i * 4,
                2 + i * 4,
                1 + i * 4,
                3 + i * 4,
            ]);
        }
        ebo.update();

        Quad {
            ebo: Rc::new(ebo),
            vbo: Rc::new(vbo),
        }
    }
}
