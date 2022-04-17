use crate::game::Value;
use crate::graphics::util::rect;
use crate::graphics::{Instancing::*, Object, Quad, VertexBuffer};
use crate::layout::Layout;

pub struct Tiles {
    obj: Object,
    rects: VertexBuffer<u32>,
    colours: VertexBuffer<f32>,
    num_instances: u32,
}

#[derive(Debug)]
pub struct Tile {
    pub pos: (usize, usize),
    pub value: Option<Value>,
    pub colour: [f32; 3],
    pub rect: [u32; 4],
}

impl Tiles {
    pub fn new(gl: &glow::Context) -> Self {
        let mut quad = Quad::new(
            gl,
            include_bytes!("../shaders/tile.v.glsl"),
            include_bytes!("../shaders/tile.f.glsl"),
        );

        let rects = VertexBuffer::new(gl, 4, ByInstance);
        quad.vao.add_buffer(gl, rects.clone());

        let colours = VertexBuffer::new(gl, 3, ByInstance);
        quad.vao.add_buffer(gl, colours.clone());

        Tiles {
            obj: quad.into_object(None),
            rects,
            colours,
            num_instances: 0,
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.obj.cleanup(gl);
    }

    pub unsafe fn render(&self, gl: &glow::Context) {
        self.obj.render(gl, self.num_instances);
    }

    pub fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) {
        self.obj.program().set_uniform(
            gl,
            "viewport",
            rect(0, 0, width as i32, height as i32),
        );
    }

    pub fn update(
        &mut self,
        gl: &glow::Context,
        _layout: &Layout,
        tiles: &[Tile],
    ) {
        self.rects.buffer.truncate(0);
        self.colours.buffer.truncate(0);

        for tile in tiles {
            self.colours.buffer.extend_from_slice(&tile.colour);
            self.rects.buffer.extend_from_slice(&tile.rect);
        }

        self.rects.update(gl, glow::STATIC_DRAW);
        self.colours.update(gl, glow::STATIC_DRAW);

        self.num_instances = tiles.len() as u32;
    }
}
