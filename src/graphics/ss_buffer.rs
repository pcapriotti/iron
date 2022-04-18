use glow::HasContext;
use std::rc::Rc;

pub struct ShaderStorageBuffer {
    gl: Rc<glow::Context>,
    pub(super) inner: glow::NativeBuffer,
}

impl ShaderStorageBuffer {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        let ssbo = unsafe { gl.create_buffer().unwrap() };
        Self { gl, inner: ssbo }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        unsafe {
            self.gl
                .bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(self.inner));
            self.gl.buffer_data_u8_slice(
                glow::SHADER_STORAGE_BUFFER,
                data,
                glow::STATIC_DRAW,
            );
            self.gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, None);
        }
    }

    pub fn cleanup(&mut self) {
        unsafe { self.gl.delete_buffer(self.inner) };
    }

    pub fn bind(&self, index: u32) {
        unsafe {
            self.gl.bind_buffer_base(
                glow::SHADER_STORAGE_BUFFER,
                index,
                Some(self.inner),
            );
        }
    }
}
