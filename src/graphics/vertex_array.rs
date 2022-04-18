use super::vertex_buffer::VertexBufferRef;
use glow::HasContext;
use std::rc::Rc;

pub struct VertexArray {
    gl: Rc<glow::Context>,
    inner: glow::NativeVertexArray,
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

    pub fn add_buffer(&mut self, buffer: Rc<VertexBufferRef>) {
        buffer.enable(&self.bind(), self.buffers.len() as u32);
        self.buffers.push(buffer);
    }

    pub fn bind<'a>(&'a self) -> BoundVertexArray<'a> {
        BoundVertexArray::new(self)
    }
}

pub struct BoundVertexArray<'a> {
    vao: &'a VertexArray,
}

impl<'a> BoundVertexArray<'a> {
    fn new(vao: &'a VertexArray) -> BoundVertexArray<'a> {
        unsafe { vao.gl.bind_vertex_array(Some(vao.inner)) };
        BoundVertexArray { vao }
    }
}

impl<'a> Drop for BoundVertexArray<'a> {
    fn drop(&mut self) {
        unsafe {
            self.vao.gl.bind_vertex_array(None);
        }
    }
}
