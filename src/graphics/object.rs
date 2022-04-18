use super::element_buffer::ElementBufferRef;
use super::shader::Program;
use super::texture::Texture;
use super::vertex_array::VertexArray;
use std::rc::Rc;

use glow::HasContext;

pub struct Object {
    vao: VertexArray,
    ebo: Rc<ElementBufferRef>,
    texture: Option<Texture>,
    program: Program,
}

impl Object {
    pub fn new(
        vao: VertexArray,
        ebo: Rc<ElementBufferRef>,
        texture: Option<Texture>,
        program: Program,
    ) -> Self {
        Self {
            vao,
            ebo,
            texture,
            program,
        }
    }

    pub unsafe fn render(&self, num: u32) {
        if num <= 0 {
            return;
        };
        let gl = self.vao.context();
        let _bvao = self.vao.bind();

        gl.use_program(Some(self.program.inner));
        let _bebo = self.ebo.bind();
        let _btex = self.texture.as_ref().map(|t| t.bind());
        gl.draw_elements(
            glow::TRIANGLES,
            num as i32 * 6,
            glow::UNSIGNED_INT,
            0,
        );
        gl.use_program(None);
    }

    pub fn program(&mut self) -> &mut Program {
        &mut self.program
    }
}
