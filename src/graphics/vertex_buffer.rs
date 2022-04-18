use glow::HasContext;
use std::rc::Rc;

pub trait GL: bytemuck::Pod {
    fn ty() -> u32;
}

impl GL for i32 {
    fn ty() -> u32 {
        glow::INT
    }
}

impl GL for u32 {
    fn ty() -> u32 {
        glow::INT
    }
}

impl GL for f32 {
    fn ty() -> u32 {
        glow::FLOAT
    }
}

pub struct VertexBufferRef {
    gl: Rc<glow::Context>,
    inner: glow::NativeBuffer,
}

impl VertexBufferRef {
    fn new(gl: Rc<glow::Context>) -> VertexBufferRef {
        let vbo = unsafe { gl.create_buffer().unwrap() };
        VertexBufferRef { gl, inner: vbo }
    }
}

impl std::ops::Deref for VertexBufferRef {
    type Target = glow::NativeBuffer;

    fn deref(&self) -> &glow::NativeBuffer {
        &self.inner
    }
}

impl Drop for VertexBufferRef {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.inner);
        }
    }
}

#[derive(Clone)]
pub struct VertexBuffer<T> {
    pub inner: Rc<VertexBufferRef>,
    pub size: i32,
    pub buffer: Vec<T>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: GL> VertexBuffer<T> {
    pub fn new(gl: Rc<glow::Context>, size: i32) -> Self {
        let inner = Rc::new(VertexBufferRef::new(gl));
        Self {
            inner,
            size,
            buffer: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn enable(&self, i: u32) {
        unsafe {
            self.inner
                .gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.inner.inner));
            match T::ty() {
                glow::INT => self.inner.gl.vertex_attrib_pointer_i32(
                    i,
                    self.size,
                    T::ty(),
                    0,
                    0,
                ),
                glow::FLOAT => self.inner.gl.vertex_attrib_pointer_f32(
                    i,
                    self.size,
                    T::ty(),
                    false,
                    0,
                    0,
                ),
                _ => panic!("Unsupported VertexBuffer type {}", T::ty()),
            };
            self.inner.gl.enable_vertex_attrib_array(i);
        }
    }

    pub fn update(&mut self, usage: u32) {
        unsafe {
            self.inner
                .gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.inner.inner));
            self.inner.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.buffer),
                usage,
            );
            self.inner.gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }
}
