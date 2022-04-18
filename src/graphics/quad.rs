use super::element_buffer::{ElementBuffer, ElementBufferRef};
use super::vertex_buffer::{VertexBuffer, VertexBufferRef};
use std::rc::Rc;

pub const VERTICES: [f32; 8] = [
    0.0, 0.0, // bottom left
    1.0, 0.0, // bottom right
    0.0, 1.0, // top left
    1.0, 1.0, // top right
];

const MIN_SIZE: u32 = 32;

pub struct Quad {
    ebo: ElementBuffer,
    vbo: VertexBuffer<f32>,
    size: u32,
}

impl Quad {
    pub fn new(gl: Rc<glow::Context>) -> Quad {
        let ebo = ElementBuffer::new(gl.clone());
        let vbo = VertexBuffer::new(gl, 2);
        let mut quad = Quad { ebo, vbo, size: 0 };

        quad.ensure(MIN_SIZE);
        quad
    }

    pub fn ebo(&self) -> Rc<ElementBufferRef> {
        self.ebo.to_ref()
    }

    pub fn vbo(&self) -> Rc<VertexBufferRef> {
        self.vbo.to_ref()
    }

    pub fn ensure(&mut self, num: u32) {
        if num <= self.size {
            return;
        }

        for i in self.size..num as u32 {
            self.ebo.buffer.extend_from_slice(&[
                i * 4,
                1 + i * 4,
                2 + i * 4,
                2 + i * 4,
                1 + i * 4,
                3 + i * 4,
            ]);
        }
        self.ebo.update();

        for _ in self.size..num {
            self.vbo.buffer.extend_from_slice(&VERTICES);
        }
        self.vbo.update(glow::STATIC_DRAW);
    }
}
