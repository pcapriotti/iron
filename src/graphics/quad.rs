use crate::graphics::{
    ElementBuffer, Instancing::*, Object, Program, Texture, VertexArray,
    VertexBuffer,
};

pub struct Quad {
    pub vao: VertexArray,
    pub ebo: ElementBuffer,
    pub program: Program,
}

const VERTICES: [f32; 8] = [
    0.0, 0.0, // bottom left
    1.0, 0.0, // bottom right
    0.0, 1.0, // top left
    1.0, 1.0, // top right
];

const INDICES: [u32; 6] = [
    0, 1, 2, // bottom left
    2, 1, 3, // top right
];

impl Quad {
    pub fn new(gl: &glow::Context, vert: &[u8], frag: &[u8]) -> Self {
        let program = Program::new(gl, vert, frag);
        let mut vbo: VertexBuffer<f32> = VertexBuffer::new(gl, 2, ByVertex);

        vbo.buffer.extend_from_slice(&VERTICES);
        vbo.update(gl, glow::STATIC_DRAW);
        vbo.buffer.truncate(0);

        let mut vao = VertexArray::new(gl);
        vao.add_buffer(gl, vbo);
        let mut ebo = ElementBuffer::new(gl);
        ebo.set_data(gl, &INDICES);

        Quad { vao, ebo, program }
    }

    pub fn into_object(self, texture: Option<Texture>) -> Object {
        let Quad { vao, ebo, program } = self;
        Object::new(vao, ebo, texture, program)
    }
}
