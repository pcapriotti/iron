use crate::game::Game;
use crate::graphics::util::rect;
use crate::graphics::{Instancing::*, Object, Quad, VertexBuffer};
use crate::layout::Layout;

pub struct Tiles {
    obj: Object,
    rects: VertexBuffer,
    colours: VertexBuffer,
    num_instances: u32,
}

impl Tiles {
    pub fn new(gl: &glow::Context) -> Self {
        let mut quad = Quad::new(
            gl,
            include_bytes!("../shaders/tile.v.glsl"),
            include_bytes!("../shaders/tile.f.glsl"),
        );

        let rects = VertexBuffer::new(gl, 4, glow::INT, ByInstance);
        quad.vao.add_buffer(gl, rects.clone());

        let colours = VertexBuffer::new(gl, 3, glow::FLOAT, ByInstance);
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

    pub fn update(&mut self, gl: &glow::Context, layout: &Layout, game: &Game) {
        let mut rects: Vec<u32> = Vec::new();
        let mut colours: Vec<f32> = Vec::new();

        let mut count = 0;
        for ((x, y), value) in game.all_tiles() {
            rects.extend_from_slice(&[
                layout.origin.0 + x as u32 * layout.unit + layout.gap,
                layout.origin.1 + y as u32 * layout.unit + layout.gap,
                layout.unit - 2 * layout.gap,
                layout.unit - 2 * layout.gap,
            ]);
            colours.extend_from_slice(match value {
                None => &[0.8, 0.8, 0.8],
                Some(0) => &[0.5, 0.7, 0.88],
                Some(1) => &[0.25, 0.6, 0.82],
                Some(2) => &[0.0, 0.0, 1.0],
                _ => &[0.2, 0.2, 0.3],
            });
            count += 1;
        }

        self.rects.set_data(gl, &rects, glow::STATIC_DRAW);
        self.colours.set_data(gl, &colours, glow::STATIC_DRAW);
        self.num_instances = count;
    }
}
