use super::vertex_array::BoundVertexArray;
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
    size: i32,
    ty: u32,
}

impl VertexBufferRef {
    fn new(gl: Rc<glow::Context>, size: i32, ty: u32) -> VertexBufferRef {
        let vbo = unsafe { gl.create_buffer().unwrap() };
        VertexBufferRef {
            gl,
            inner: vbo,
            size,
            ty,
        }
    }

    pub fn enable(&self, _bvao: &BoundVertexArray, i: u32) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.inner));
            match self.ty {
                glow::INT => self
                    .gl
                    .vertex_attrib_pointer_i32(i, self.size, self.ty, 0, 0),
                glow::FLOAT => self.gl.vertex_attrib_pointer_f32(
                    i, self.size, self.ty, false, 0, 0,
                ),
                _ => panic!("Unsupported VertexBuffer type {}", self.ty),
            };
            self.gl.enable_vertex_attrib_array(i);
        }
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
    inner: Rc<VertexBufferRef>,
    pub buffer: Vec<T>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: GL> VertexBuffer<T> {
    pub fn new(gl: Rc<glow::Context>, size: i32) -> Self {
        let inner = Rc::new(VertexBufferRef::new(gl, size, T::ty()));
        Self {
            inner,
            buffer: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn to_ref(&self) -> Rc<VertexBufferRef> {
        self.inner.clone()
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
