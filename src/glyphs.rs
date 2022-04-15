use crate::game::Game;
use crate::graphics::util::rect;
use crate::graphics::{
    ElementBuffer, GlyphCache, Instancing::*, Object, Program, VertexArray,
    VertexBuffer,
};

pub struct Glyphs {
    obj: Object,

    cell_rects: VertexBuffer,
    glyph_indices: VertexBuffer,
    cache: GlyphCache,
    num_instances: u32,
    width: u32,
    height: u32,
}

impl Glyphs {
    const GAP: f32 = 0.03;

    pub fn new(gl: &glow::Context) -> Self {
        let vertices: [f32; 8] = [
            0.0, 0.0, // bottom left
            1.0, 0.0, // bottom right
            0.0, 1.0, // top left
            1.0, 1.0, // top right
        ];
        let indices: [u32; 6] = [
            0, 1, 2, // bottom left
            2, 1, 3, // top right
        ];

        // vertices
        let mut vbo = VertexBuffer::new(gl, 2, glow::FLOAT, ByVertex);
        vbo.set_data(gl, &vertices[..], glow::STATIC_DRAW);

        // cell rects
        let cell_rects = VertexBuffer::new(gl, 4, glow::INT, ByInstance);

        // glyph indices
        let glyph_indices = VertexBuffer::new(gl, 1, glow::INT, ByInstance);

        let vao = VertexArray::new(
            gl,
            vec![vbo, cell_rects.clone(), glyph_indices.clone()],
        );

        let mut ebo = ElementBuffer::new(gl);
        ebo.set_data(gl, &indices[..]);

        let program = Program::new(
            gl,
            include_bytes!("../shaders/glyph.v.glsl"),
            include_bytes!("../shaders/glyph.f.glsl"),
        );

        let mut cache = GlyphCache::new(gl, 0);
        let texture = cache.make_atlas(gl);

        let obj = Object::new(vao, ebo, texture, program);

        Self {
            obj,
            cell_rects,
            glyph_indices,
            cache,
            num_instances: 1,
            width: 0,
            height: 0,
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.obj.cleanup(gl);
        self.cache.cleanup(gl);
    }

    pub unsafe fn render(&self, gl: &glow::Context) {
        self.obj.render(gl, self.num_instances);
    }

    pub fn setup_grid(&mut self, gl: &glow::Context, game: &Game) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        self.num_instances = (game.width() * game.height()) as u32;

        let mut cell_rects: Vec<i32> = Vec::new();
        let mut glyph_indices: Vec<i32> = Vec::new();

        let (gap, cell_size) = if self.width < self.height {
            let gap = (self.width as f32 * Self::GAP) as i32;
            (
                gap,
                ((self.width as f32 - gap as f32) / game.width() as f32) as i32
                    - gap,
            )
        } else {
            let gap = (self.height as f32 * Self::GAP) as i32;
            (
                gap,
                ((self.height as f32 - gap as f32) / game.height() as f32)
                    as i32
                    - gap,
            )
        };

        for y in 0..game.height() as i32 {
            for x in 0..game.width() as i32 {
                let r = [
                    gap + x * (gap + cell_size),
                    gap + y * (gap + cell_size),
                    cell_size,
                    cell_size,
                ];
                cell_rects.extend_from_slice(&r);
                glyph_indices.push(65 + x + 4 * y);
            }
        }

        self.cell_rects
            .set_data(gl, &cell_rects[..], glow::DYNAMIC_DRAW);
        self.glyph_indices
            .set_data(gl, &glyph_indices[..], glow::DYNAMIC_DRAW);

        self.cache.upload_atlas(gl, &self.obj.texture.bind(gl));
    }

    pub fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.obj.program.set_uniform(
            gl,
            "viewport",
            rect(0, 0, width as i32, height as i32),
        );
    }
}
