use glow::HasContext;
use std::rc::Rc;

pub struct ElementBuffer {
    gl: Rc<glow::Context>,
    pub(super) inner: glow::NativeBuffer,
    pub(super) size: usize,
}

impl ElementBuffer {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        let ebo = unsafe { gl.create_buffer().unwrap() };
        Self {
            gl,
            inner: ebo,
            size: 0,
        }
    }

    pub fn set_data(&mut self, data: &[u32]) {
        self.size = data.len();
        unsafe {
            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.inner));
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(data),
                glow::STATIC_DRAW,
            );
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }

    pub fn cleanup(&mut self) {
        unsafe { self.gl.delete_buffer(self.inner) };
    }
}
