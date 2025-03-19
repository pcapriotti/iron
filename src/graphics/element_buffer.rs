use glow::HasContext;
use std::rc::Rc;

pub struct ElementBufferRef {
    gl: Rc<glow::Context>,
    inner: glow::NativeBuffer,
}

impl ElementBufferRef {
    fn new(gl: Rc<glow::Context>) -> ElementBufferRef {
        let ebo = unsafe { gl.create_buffer().unwrap() };
        ElementBufferRef { gl, inner: ebo }
    }

    pub fn bind<'a>(&'a self) -> BoundElementBuffer<'a> {
        BoundElementBuffer::new(self)
    }
}

impl Drop for ElementBufferRef {
    fn drop(&mut self) {
        unsafe { self.gl.delete_buffer(self.inner) };
    }
}

pub struct ElementBuffer {
    inner: Rc<ElementBufferRef>,
    pub(super) buffer: Vec<u32>,
}

impl ElementBuffer {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self {
            inner: Rc::new(ElementBufferRef::new(gl)),
            buffer: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        unsafe {
            self.inner
                .gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.inner.inner));
            self.inner.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.buffer),
                glow::STATIC_DRAW,
            );
            self.inner.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }

    pub fn to_ref(&self) -> Rc<ElementBufferRef> {
        self.inner.clone()
    }
}

pub struct BoundElementBuffer<'a> {
    ebo: &'a ElementBufferRef,
}

impl<'a> BoundElementBuffer<'a> {
    fn new(ebo: &'a ElementBufferRef) -> BoundElementBuffer {
        unsafe {
            ebo.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo.inner));
        }
        BoundElementBuffer { ebo }
    }
}

impl<'a> Drop for BoundElementBuffer<'a> {
    fn drop(&mut self) {
        unsafe {
            self.ebo.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }
}
