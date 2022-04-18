use crate::game::Value;
use crate::graphics::util::rect;
use crate::graphics::{
    quad, ElementBuffer, Object, Program, VertexArray, VertexBuffer,
};
use std::rc::Rc;

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
    pub fn new(gl: Rc<glow::Context>, max_tiles: usize) -> Self {
        let program = Program::new(
            gl.clone(),
            include_bytes!("../shaders/tile.v.glsl"),
            include_bytes!("../shaders/tile.f.glsl"),
        );
        let mut vao = VertexArray::new(gl.clone());

        let mut vertices: VertexBuffer<f32> = VertexBuffer::new(gl.clone(), 2);
        // prefill vertices
        for _ in 0..max_tiles {
            vertices.buffer.extend_from_slice(&quad::VERTICES);
        }
        vertices.update(glow::STATIC_DRAW);
        vao.add_buffer(vertices);

        let rects: VertexBuffer<u32> = VertexBuffer::new(gl.clone(), 2);
        vao.add_buffer(rects.clone());

        let mut ebo = ElementBuffer::new(gl.clone());
        let mut ebo_buffer = Vec::new();
        // prefill indices
        for i in 0..max_tiles as u32 {
            ebo_buffer.extend_from_slice(&[
                i * 4,
                1 + i * 4,
                2 + i * 4,
                2 + i * 4,
                1 + i * 4,
                3 + i * 4,
            ]);
        }
        ebo.set_data(&ebo_buffer[..]);

        let colours = VertexBuffer::new(gl.clone(), 3);
        vao.add_buffer(colours.clone());

        Tiles {
            obj: Object::new(vao, ebo, None, program),
            rects,
            colours,
            num_instances: 0,
        }
    }

    pub unsafe fn render(&self) {
        self.obj.render(self.num_instances);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.obj
            .program()
            .set_uniform("viewport", rect(0, 0, width as i32, height as i32));
    }

    pub fn update(&mut self, tiles: &[Tile]) {
        self.rects.buffer.truncate(0);
        self.colours.buffer.truncate(0);
        let mut count = 0;

        for tile in tiles {
            for _ in 0..4 {
                self.colours.buffer.extend_from_slice(&tile.colour);
            }

            self.rects.buffer.push(tile.rect[0]);
            self.rects.buffer.push(tile.rect[1]);
            self.rects.buffer.push(tile.rect[0] + tile.rect[2]);
            self.rects.buffer.push(tile.rect[1]);
            self.rects.buffer.push(tile.rect[0]);
            self.rects.buffer.push(tile.rect[1] + tile.rect[3]);
            self.rects.buffer.push(tile.rect[0] + tile.rect[2]);
            self.rects.buffer.push(tile.rect[1] + tile.rect[3]);

            count += 1;
        }

        self.rects.update(glow::STATIC_DRAW);
        self.colours.update(glow::STATIC_DRAW);
        self.num_instances = count;
    }
}
