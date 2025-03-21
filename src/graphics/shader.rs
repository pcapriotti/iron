use super::uniform::UniformValue;
use glow::HasContext;
use std::rc::Rc;

pub struct Program {
    gl: Rc<glow::Context>,
    pub inner: glow::NativeProgram,
}

impl Program {
    pub fn new(gl: Rc<glow::Context>, vert: &[u8], frag: &[u8]) -> Self {
        let prog = unsafe {
            let vert = compile_shader_from_source(&gl, glow::VERTEX_SHADER, vert);

            let frag = compile_shader_from_source(&gl, glow::FRAGMENT_SHADER, frag);

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
        };

        Self { gl, inner: prog }
    }

    pub fn set_uniform(&mut self, name: &str, value: impl UniformValue) {
        unsafe {
            self.gl.use_program(Some(self.inner));
            let loc = self.gl.get_uniform_location(self.inner, name);
            value.set(&self.gl, loc.as_ref());
            self.gl.use_program(None);
        }
    }

    pub fn bind<'a>(&'a self) -> BoundProgram<'a> {
        BoundProgram::new(self)
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

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.inner) };
    }
}

pub struct BoundProgram<'a> {
    program: &'a Program,
}

impl<'a> BoundProgram<'a> {
    fn new(program: &'a Program) -> BoundProgram<'a> {
        unsafe { program.gl.use_program(Some(program.inner)) };
        BoundProgram { program }
    }
}

impl<'a> Drop for BoundProgram<'a> {
    fn drop(&mut self) {
        unsafe {
            self.program.gl.use_program(None);
        }
    }
}
