use super::element_buffer::{BoundElementBuffer, ElementBufferRef};
use super::shader::Program;
use super::texture::{BoundTexture, Texture};
use super::vertex_array::{BoundVertexArray, VertexArray};
use std::rc::Rc;

use glow::HasContext;

pub struct Object {
    gl: Rc<glow::Context>,
    vao: VertexArray,
    ebo: Rc<ElementBufferRef>,
    texture: Option<Texture>,
    program: Program,
}

pub fn render_object(
    gl: &glow::Context,
    _bvao: &BoundVertexArray,
    _bebo: &BoundElementBuffer,
    _btex: &Option<BoundTexture>,
    program: &Program,
    num: u32,
) {
    unsafe {
        gl.use_program(Some(program.inner));
        gl.draw_elements(
            glow::TRIANGLES,
            num as i32 * 6,
            glow::UNSIGNED_INT,
            0,
        );
        gl.use_program(None);
    }
}

impl Object {
    pub fn new(
        gl: Rc<glow::Context>,
        vao: VertexArray,
        ebo: Rc<ElementBufferRef>,
        texture: Option<Texture>,
        program: Program,
    ) -> Self {
        Self {
            gl,
            vao,
            ebo,
            texture,
            program,
        }
    }

    pub fn render(&self, num: u32) {
        if num > 0 {
            let btex = self.texture.as_ref().map(|t| t.bind());
            render_object(
                &self.gl,
                &self.vao.bind(),
                &self.ebo.bind(),
                &btex,
                &self.program,
                num,
            );
        }
    }

    pub fn program(&mut self) -> &mut Program {
        &mut self.program
    }
}
