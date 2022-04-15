use crate::game::Game;
use crate::graphics::util::rect;
use crate::graphics::{Instancing::*, Object, Quad, VertexBuffer};

pub struct Tiles {
    obj: Object,
    rects: VertexBuffer,
    width: u32,
    height: u32,
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

        Tiles {
            obj: quad.into_object(None),
            rects,
            width: 0,
            height: 0,
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
        self.width = width;
        self.height = height;
        self.obj.program().set_uniform(
            gl,
            "viewport",
            rect(0, 0, width as i32, height as i32),
        );
    }

    pub fn update(&mut self, gl: &glow::Context, game: &Game) {
        let mut rects: Vec<u32> = Vec::new();

        let unit = std::cmp::min(
            self.width / game.width() as u32,
            self.height / game.height() as u32,
        );
        let gap = (unit as f32 * 0.07) as u32;
        let display_width = game.width() as u32 * unit;
        let display_height = game.height() as u32 * unit;

        let x0 = (self.width - display_width) / 2;
        let y0 = (self.height - display_height) / 2;

        let mut count = 0;
        for (i, _value) in game.tiles() {
            let x = i % game.width();
            let y = i / game.width();

            rects.extend_from_slice(&[
                x0 + x as u32 * unit + gap,
                y0 + y as u32 * unit + gap,
                unit - 2 * gap,
                unit - 2 * gap,
            ]);
            count += 1;
        }

        self.rects.set_data(gl, &rects, glow::STATIC_DRAW);
        self.num_instances = count;
    }
}
