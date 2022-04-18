use super::element_buffer::ElementBuffer;
use super::shader::Program;
use super::texture::Texture;
use super::vertex_array::VertexArray;

use glow::HasContext;

pub struct Object {
    vao: VertexArray,
    ebo: ElementBuffer,
    texture: Option<Texture>,
    program: Program,
}

impl Object {
    pub fn new(
        vao: VertexArray,
        ebo: ElementBuffer,
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

    pub fn cleanup(&mut self) {
        self.vao.cleanup();
        self.ebo.cleanup();
        self.program.cleanup();
        if let Some(texture) = &mut self.texture {
            texture.cleanup();
        }
    }

    pub unsafe fn render(&self, num: u32) {
        if num <= 0 {
            return;
        };
        self.vao.gl.use_program(Some(self.program.inner));
        self.vao.gl.bind_vertex_array(Some(self.vao.inner));
        self.vao
            .gl
            .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo.inner));
        let _btex = self.texture.as_ref().map(|t| t.bind());
        self.vao.gl.draw_elements(
            glow::TRIANGLES,
            num as i32 * 6,
            glow::UNSIGNED_INT,
            0,
        );
        self.vao.gl.bind_vertex_array(None);
        self.vao.gl.use_program(None);
    }

    pub fn program(&mut self) -> &mut Program {
        &mut self.program
    }
}
