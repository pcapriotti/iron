use super::vertex_buffer::{Instancing, VertexBuffer};
use glow::HasContext;

pub struct VertexArray {
    pub inner: glow::NativeVertexArray,
    buffers: Vec<VertexBuffer>,
}

impl VertexArray {
    pub fn new(gl: &glow::Context, buffers: Vec<VertexBuffer>) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };
        unsafe { gl.bind_vertex_array(Some(vao)) };

        for (i, buffer) in buffers.iter().enumerate() {
            buffer.enable(gl, i as u32);
        }
        unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, None) };

        unsafe { gl.bind_vertex_array(None) };
        Self {
            inner: vao,
            buffers,
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        for buffer in self.buffers.iter_mut() {
            buffer.cleanup(gl);
        }
    }
}
