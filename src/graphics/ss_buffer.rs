use glow::HasContext;

pub struct ShaderStorageBuffer {
    pub(super) inner: glow::NativeBuffer,
}

impl ShaderStorageBuffer {
    pub fn new(gl: &glow::Context) -> Self {
        let ssbo = unsafe { gl.create_buffer().unwrap() };
        Self { inner: ssbo }
    }

    pub fn set_data(&mut self, gl: &glow::Context, data: &[u8]) {
        unsafe {
            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(self.inner));
            gl.buffer_data_u8_slice(
                glow::SHADER_STORAGE_BUFFER,
                data,
                glow::STATIC_DRAW,
            );
            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, None);
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        unsafe { gl.delete_buffer(self.inner) };
    }

    pub fn bind(&self, gl: &glow::Context, index: u32) {
        unsafe {
            gl.bind_buffer_base(
                glow::SHADER_STORAGE_BUFFER,
                index,
                Some(self.inner),
            );
        }
    }
}
