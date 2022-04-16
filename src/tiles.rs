use crate::game::{Game, Move};
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

    pub fn update(
        &mut self,
        gl: &glow::Context,
        layout: &Layout,
        game: &Game,
        moves: &[Move],
        time: f32,
    ) {
        let mut rects: Vec<u32> = Vec::new();
        let mut colours: Vec<f32> = Vec::new();
        let mut count = 0;

        for (pos, _) in game.all_tiles() {
            colours.extend_from_slice(&[0.2, 0.2, 0.2]);
            rects.extend_from_slice(&layout.rect(pos));
            count += 1;
        }

        for (pos, value) in game.tiles() {
            let colour = match value % 10 {
                1 => [0.9453125, 0.6171875, 0.296875],
                2 => [0.94140625, 0.765625, 0.32421875],
                3 => [0.93359375, 0.9140625, 0.3515625],
                4 => [0.72265625, 0.90234375, 0.41015625],
                5 => [0.51171875, 0.88671875, 0.46484375],
                6 => [0.0859375, 0.85546875, 0.57421875],
                7 => [0.05078125, 0.69921875, 0.6171875],
                8 => [0.015625, 0.54296875, 0.65625],
                9 => [0.171875, 0.41015625, 0.6015625],
                0 => [0.328125, 0.27734375, 0.546875],
                _ => unreachable!(),
            };
            colours.extend_from_slice(&colour);

            let mut rect = layout.rect(pos);
            let (delta_x, delta_y) = layout.animate(moves, pos.0, pos.1, time);
            rect[0] = (rect[0] as i32 - delta_x) as u32;
            rect[1] = (rect[1] as i32 - delta_y) as u32;
            rects.extend_from_slice(&rect);
            count += 1;
        }

        self.rects.set_data(gl, &rects, glow::STATIC_DRAW);
        self.colours.set_data(gl, &colours, glow::STATIC_DRAW);
        self.num_instances = count;
    }
}
