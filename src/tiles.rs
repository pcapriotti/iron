use crate::graphics::{Object, Quad};

pub struct Tiles {
    obj: Object,
}

impl Tiles {
    pub fn new(gl: &glow::Context) -> Self {
        let quad = Quad::new(
            gl,
            include_bytes!("../shaders/tile.v.glsl"),
            include_bytes!("../shaders/tile.f.glsl"),
        );
        Tiles {
            obj: quad.into_object(None),
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.obj.cleanup(gl);
    }

    pub unsafe fn render(&self, gl: &glow::Context) {
        self.obj.render(gl, 1);
    }
}
