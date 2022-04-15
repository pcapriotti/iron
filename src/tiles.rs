use crate::graphics::Object;

pub struct Tiles {
    obj: Object,
}
impl Tiles {
    pub fn new(gl: &glow::Context) -> Self {
        let vao = todo!();
        let ebo = todo!();
        let program = todo!();
        let obj = Object::new(vao, ebo, None, program);
    }
}
