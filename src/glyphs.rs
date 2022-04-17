use crate::graphics::util::rect;
use crate::graphics::{
    GlyphCache, GlyphInfo, Instancing::*, Object, Quad, VertexBuffer,
};
use crate::layout::Layout;
use crate::tiles::Tile;

pub struct Glyphs {
    obj: Object,

    cell_rects: VertexBuffer<u32>,
    glyph_indices: VertexBuffer<u32>,

    cache: GlyphCache,
    infos: Vec<GlyphInfo<'static>>,
    num_instances: u32,
}

impl Glyphs {
    pub fn new(gl: &glow::Context) -> Self {
        let mut quad = Quad::new(
            gl,
            include_bytes!("../shaders/glyph.v.glsl"),
            include_bytes!("../shaders/glyph.f.glsl"),
        );

        // cell rects
        let cell_rects = VertexBuffer::new(gl, 4, ByInstance);

        // glyph indices
        let glyph_indices = VertexBuffer::new(gl, 1, ByInstance);

        quad.vao.add_buffer(gl, cell_rects.clone());
        quad.vao.add_buffer(gl, glyph_indices.clone());

        let mut cache = GlyphCache::new(gl, 0);
        let (infos, texture) = cache.make_atlas(gl);
        cache.upload_atlas(gl, &texture.bind(gl));

        let obj = quad.into_object(Some(texture));

        Self {
            obj,
            cell_rects,
            glyph_indices,
            cache,
            infos,
            num_instances: 0,
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.obj.cleanup(gl);
        self.cache.cleanup(gl);
    }

    pub unsafe fn render(&self, gl: &glow::Context) {
        self.obj.render(gl, self.num_instances);
    }

    pub fn update(
        &mut self,
        gl: &glow::Context,
        layout: &Layout,
        tiles: &[Tile],
    ) {
        self.cell_rects.buffer.truncate(0);
        self.glyph_indices.buffer.truncate(0);

        let mut count = 0;

        for t in tiles {
            if let Tile {
                value: Some(value),
                rect,
                ..
            } = t
            {
                let value = format!("{}", (1 as u64) << value);
                let scale = if value.len() <= 4 {
                    0.4
                } else if value.len() <= 6 {
                    0.28
                } else if value.len() <= 8 {
                    0.21
                } else {
                    0.15
                };
                let unit = (layout.unit as f32 * scale) as u32;

                // layout text
                let mut x_offsets = Vec::new();
                let mut text_width = 0;
                for d in value.chars() {
                    let index = self.cache.index_of(d);
                    self.glyph_indices.buffer.push(index as u32);

                    x_offsets.push(text_width);
                    let width = {
                        let glyph = self.infos[index].glyph().unpositioned();
                        let width = glyph.h_metrics().advance_width;
                        width / glyph.scale().x * unit as f32
                    };
                    text_width += width as u32;
                }

                let margin = (
                    ((layout.unit as i32 - text_width as i32) / 2).max(0)
                        as u32,
                    (layout.unit - unit) / 2,
                );

                for i in 0..value.len() {
                    let x = rect[0] + margin.0 + x_offsets[i] - layout.gap;
                    let y = rect[1] + margin.1 - layout.gap;
                    self.cell_rects.buffer.extend_from_slice(&[
                        x,
                        y,
                        unit as u32,
                        unit as u32,
                    ]);
                }

                count += value.len() as u32;
            }
        }

        self.num_instances = count;
        self.cell_rects.update(gl, glow::STATIC_DRAW);
        self.glyph_indices.update(gl, glow::STATIC_DRAW);
    }

    pub fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) {
        self.obj.program().set_uniform(
            gl,
            "viewport",
            rect(0, 0, width as i32, height as i32),
        );
    }
}
