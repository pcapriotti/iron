use crate::game::Game;
use crate::graphics::util::rect;
use crate::graphics::{GlyphCache, Instancing::*, Object, Quad, VertexBuffer};
use crate::layout::Layout;

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
    pub fn new(gl: &glow::Context) -> Self {
        let mut quad = Quad::new(
            gl,
            include_bytes!("../shaders/glyph.v.glsl"),
            include_bytes!("../shaders/glyph.f.glsl"),
        );

        // cell rects
        let cell_rects = VertexBuffer::new(gl, 4, glow::INT, ByInstance);

        // glyph indices
        let glyph_indices = VertexBuffer::new(gl, 1, glow::INT, ByInstance);

        quad.vao.add_buffer(gl, cell_rects.clone());
        quad.vao.add_buffer(gl, glyph_indices.clone());

        let mut cache = GlyphCache::new(gl, 0);
        let texture = cache.make_atlas(gl);
        cache.upload_atlas(gl, &texture.bind(gl));

        let obj = quad.into_object(Some(texture));

        Self {
            obj,
            cell_rects,
            glyph_indices,
            cache,
            num_instances: 0,
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

    pub fn update(&mut self, gl: &glow::Context, layout: &Layout, game: &Game) {
        let mut cell_rects: Vec<u32> = Vec::new();
        let mut glyph_indices: Vec<u32> = Vec::new();

        let unit = (layout.unit as f32 * 0.28) as u32;
        let margin = ((layout.unit - unit) / 2, (layout.unit - unit) / 2);

        let mut count = 0;
        for ((x, y), value) in game.tiles() {
            let value = 1 << value;
            let value = value % 10; // TODO: support multiple digits

            cell_rects.extend_from_slice(&[
                layout.origin.0 + x as u32 * layout.unit + margin.0,
                layout.origin.1 + y as u32 * layout.unit + margin.1,
                unit,
                unit,
            ]);
            glyph_indices
                .push(self.cache.index_of(('0' as u8 + value) as char) as u32);
            count += 1;
        }

        self.num_instances = count;
        self.cell_rects.set_data(gl, &cell_rects, glow::STATIC_DRAW);
        self.glyph_indices
            .set_data(gl, &glyph_indices, glow::STATIC_DRAW);
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
}
