use super::uniform::UniformValue;
use glow::HasContext;

pub struct Program {
    pub inner: glow::NativeProgram,
}

impl Program {
    pub fn new(gl: &glow::Context, vert: &[u8], frag: &[u8]) -> Self {
        Self {
            inner: unsafe {
                let vert =
                    compile_shader_from_source(&gl, glow::VERTEX_SHADER, vert);

                let frag = compile_shader_from_source(
                    &gl,
                    glow::FRAGMENT_SHADER,
                    frag,
                );

                let program = gl.create_program().unwrap();
                gl.attach_shader(program, vert);
                gl.attach_shader(program, frag);
                gl.link_program(program);
                if !gl.get_program_link_status(program) {
                    panic!("link error: {}", gl.get_program_info_log(program));
                }

                gl.detach_shader(program, vert);
                gl.delete_shader(vert);

                gl.detach_shader(program, frag);
                gl.delete_shader(frag);
                program
            },
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        unsafe { gl.delete_program(self.inner) };
    }

    pub fn set_uniform(
        &mut self,
        gl: &glow::Context,
        name: &str,
        value: impl UniformValue,
    ) {
        unsafe {
            gl.use_program(Some(self.inner));
            let loc = gl.get_uniform_location(self.inner, name);
            value.set(gl, loc.as_ref());
            gl.use_program(None);
        }
    }
}

fn compile_shader_from_source(
    gl: &glow::Context,
    shader_type: u32,
    src: &[u8],
) -> glow::NativeShader {
    unsafe {
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, std::str::from_utf8(src).unwrap());
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        shader
    }
}
