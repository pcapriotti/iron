use super::vertex_buffer::{VertexBuffer, VertexBufferRef, GL};
use glow::HasContext;
use std::rc::Rc;

pub struct VertexArray {
    pub(super) gl: Rc<glow::Context>,
    pub inner: glow::NativeVertexArray,
    buffers: Vec<Rc<VertexBufferRef>>,
}

impl VertexArray {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };

        Self {
            gl,
            inner: vao,
            buffers: Vec::new(),
        }
    }

    pub fn add_buffer<T: GL>(&mut self, buffer: VertexBuffer<T>) {
        unsafe { self.gl.bind_vertex_array(Some(self.inner)) };
        buffer.enable(self.buffers.len() as u32);
        self.buffers.push(buffer.inner);
        unsafe { self.gl.bind_buffer(glow::ARRAY_BUFFER, None) };
    }
}
