use super::element_buffer::ElementBuffer;
use super::shader::Program;
use super::texture::Texture;
use super::vertex_array::VertexArray;

use glow::HasContext;

pub struct Object {
    vao: VertexArray,
    ebo: ElementBuffer,
    pub texture: Texture,
    pub program: Program,
}

impl Object {
    pub fn new(
        vao: VertexArray,
        ebo: ElementBuffer,
        texture: Texture,
        program: Program,
    ) -> Self {
        Self {
            vao,
            ebo,
            texture,
            program,
        }
    }
    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.vao.cleanup(gl);
        self.ebo.cleanup(gl);
        self.program.cleanup(gl);
        self.texture.cleanup(gl);
    }

    pub unsafe fn render(&self, gl: &glow::Context, num_instances: u32) {
        gl.use_program(Some(self.program.inner));
        gl.bind_vertex_array(Some(self.vao.inner));
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo.inner));
        let _btex = self.texture.bind(gl);
        gl.draw_elements_instanced(
            glow::TRIANGLES,
            self.ebo.size as i32,
            glow::UNSIGNED_INT,
            0,
            num_instances as i32,
        );
        gl.bind_vertex_array(None);
        gl.use_program(None);
    }
}
