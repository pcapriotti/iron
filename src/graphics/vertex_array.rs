use super::vertex_buffer::{VertexBuffer, GL};
use glow::HasContext;

pub struct VertexArray {
    pub inner: glow::NativeVertexArray,
    buffers: Vec<glow::NativeBuffer>,
}

impl VertexArray {
    pub fn new(gl: &glow::Context) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };

        Self {
            inner: vao,
            buffers: Vec::new(),
        }
    }

    pub fn add_buffer<T: GL>(
        &mut self,
        gl: &glow::Context,
        buffer: VertexBuffer<T>,
    ) {
        unsafe { gl.bind_vertex_array(Some(self.inner)) };
        buffer.enable(gl, self.buffers.len() as u32);
        self.buffers.push(buffer.inner);
        unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, None) };
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        for buf in &self.buffers {
            unsafe { gl.delete_buffer(*buf) }
        }
    }
}
