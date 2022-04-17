use glow::HasContext;

pub trait GL {
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

#[derive(Clone, Copy)]
pub enum Instancing {
    ByVertex,
    ByInstance,
}

#[derive(Clone)]
pub struct VertexBuffer<T> {
    pub inner: glow::NativeBuffer,
    pub size: i32,
    pub instancing: Instancing,
    phantom: std::marker::PhantomData<T>,
}

impl<T: GL> VertexBuffer<T> {
    pub fn new(gl: &glow::Context, size: i32, instancing: Instancing) -> Self {
        let vbo = unsafe { gl.create_buffer().unwrap() };
        Self {
            inner: vbo,
            size,
            instancing,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn enable(&self, gl: &glow::Context, i: u32) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.inner));
            match T::ty() {
                glow::INT => {
                    gl.vertex_attrib_pointer_i32(i, self.size, T::ty(), 0, 0)
                }
                glow::FLOAT => gl.vertex_attrib_pointer_f32(
                    i,
                    self.size,
                    T::ty(),
                    false,
                    0,
                    0,
                ),
                _ => panic!("Unsupported VertexBuffer type {}", T::ty()),
            };
            if let Instancing::ByInstance = self.instancing {
                gl.vertex_attrib_divisor(i, 1);
            }
            gl.enable_vertex_attrib_array(i);
        }
    }

    pub fn set_data(
        &mut self,
        gl: &glow::Context,
        data: &[impl bytemuck::Pod],
        usage: u32,
    ) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.inner));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(data),
                usage,
            );
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }

    #[allow(dead_code)]
    pub fn cleanup(&mut self, gl: &glow::Context) {
        unsafe { gl.delete_buffer(self.inner) }
    }
}
