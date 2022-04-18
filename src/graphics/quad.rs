use super::element_buffer::{ElementBuffer, ElementBufferRef};
use super::vertex_buffer::{VertexBuffer, VertexBufferRef};
use std::cell::RefCell;
use std::rc::Rc;

pub const VERTICES: [f32; 8] = [
    0.0, 0.0, // bottom left
    1.0, 0.0, // bottom right
    0.0, 1.0, // top left
    1.0, 1.0, // top right
];

const MIN_SIZE: u32 = 32;

pub struct Quad {
    ebo: RefCell<ElementBuffer>,
    vbo: RefCell<VertexBuffer<f32>>,
    size: RefCell<u32>,
}

impl Quad {
    pub fn new(gl: Rc<glow::Context>) -> Quad {
        let ebo = ElementBuffer::new(gl.clone());
        let vbo = VertexBuffer::new(gl, 2);
        let quad = Quad {
            ebo: RefCell::new(ebo),
            vbo: RefCell::new(vbo),
            size: RefCell::new(0),
        };

        quad.ensure(MIN_SIZE);
        quad
    }

    pub fn ebo(&self) -> Rc<ElementBufferRef> {
        self.ebo.borrow().to_ref()
    }

    pub fn vbo(&self) -> Rc<VertexBufferRef> {
        self.vbo.borrow().to_ref()
    }

    pub fn ensure(&self, num: u32) {
        let size = self.size.replace_with(|sz| num.max(*sz));

        if num <= size {
            return;
        }

        let mut ebo = self.ebo.borrow_mut();
        for i in size..num as u32 {
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

        let mut vbo = self.vbo.borrow_mut();
        for _ in size..num {
            vbo.buffer.extend_from_slice(&VERTICES);
        }
        vbo.update(glow::STATIC_DRAW);
    }
}
