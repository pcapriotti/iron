use glow::HasContext;
use rusttype::Rect;

type Loc<'a> = Option<&'a glow::NativeUniformLocation>;

pub trait UniformValue {
    unsafe fn set(&self, gl: &glow::Context, loc: Loc);
}

impl UniformValue for Rect<i32> {
    unsafe fn set(&self, gl: &glow::Context, loc: Loc) {
        gl.uniform_4_i32(
            loc,
            self.min.x,
            self.min.y,
            self.width(),
            self.height(),
        );
    }
}

impl UniformValue for f32 {
    unsafe fn set(&self, gl: &glow::Context, loc: Loc) {
        gl.uniform_1_f32(loc, *self);
    }
}
