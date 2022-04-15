use super::vertex_buffer::{Instancing, VertexBuffer};
use glow::HasContext;

pub struct VertexArray {
    inner: glow::NativeVertexArray,
}

impl VertexArray {
    pub fn new(gl: &glow::Context, vbos: &[VertexBuffer]) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };
        unsafe { gl.bind_vertex_array(Some(vao)) };

        for (i, vbo) in vbos.iter().enumerate() {
            vbo.enable(gl, i as u32);
        }
        unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, None) };

        unsafe { gl.bind_vertex_array(None) };
        Self { inner: vao }
    }
}
