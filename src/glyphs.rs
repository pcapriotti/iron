use crate::graphics::util::rect;
use crate::graphics::{
    quad, ElementBuffer, GlyphCache, GlyphInfo, Object, Program, VertexArray,
    VertexBuffer,
};
use crate::tiles::Tile;
use std::rc::Rc;

pub struct Glyphs {
    obj: Object,

    cell_rects: VertexBuffer<u32>,
    glyph_indices: VertexBuffer<u32>,
    num_instances: u32,

    cache: GlyphCache,
    infos: Vec<GlyphInfo<'static>>,
}

impl Glyphs {
    pub fn new(gl: Rc<glow::Context>, max_glyphs: usize) -> Self {
        let mut vao = VertexArray::new(gl.clone());

        let program = Program::new(
            gl.clone(),
            include_bytes!("../shaders/glyph.v.glsl"),
            include_bytes!("../shaders/glyph.f.glsl"),
        );

        let mut vertices: VertexBuffer<f32> = VertexBuffer::new(gl.clone(), 2);
        for _ in 0..max_glyphs {
            vertices.buffer.extend_from_slice(&quad::VERTICES);
        }
        vertices.update(glow::STATIC_DRAW);
        vertices.buffer.truncate(0);
        vao.add_buffer(vertices);

        let mut ebo = ElementBuffer::new(gl.clone());
        let mut ebo_buffer = Vec::new();
        for i in 0..max_glyphs as u32 {
            ebo_buffer.extend_from_slice(&[
                i * 4,
                1 + i * 4,
                2 + i * 4,
                2 + i * 4,
                1 + i * 4,
                3 + i * 4,
            ]);
        }
        ebo.set_data(&ebo_buffer);

        // cell rects
        let cell_rects = VertexBuffer::new(gl.clone(), 4);

        // glyph indices
        let glyph_indices = VertexBuffer::new(gl.clone(), 1);

        vao.add_buffer(cell_rects.clone());
        vao.add_buffer(glyph_indices.clone());

        let mut cache = GlyphCache::new(gl.clone(), 0);
        let (infos, texture) = cache.make_atlas();
        cache.upload_atlas(&texture.bind());

        let obj = Object::new(vao, ebo, Some(texture), program);

        Self {
            obj,
            cell_rects,
            glyph_indices,
            num_instances: 0,
            cache,
            infos,
        }
    }

    pub fn cleanup(&mut self) {
        self.obj.cleanup();
        self.cache.cleanup();
    }

    pub unsafe fn render(&self) {
        self.obj.render(self.num_instances);
    }

    pub fn update(&mut self, tiles: &[Tile]) {
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
                let unit = (t.rect[2] as f32 * scale) as u32;

                // layout text
                let mut x_offsets = Vec::new();
                let mut text_width = 0;
                for d in value.chars() {
                    let index = self.cache.index_of(d);
                    for _ in 0..4 {
                        self.glyph_indices.buffer.push(index as u32);
                    }

                    x_offsets.push(text_width);
                    let width = {
                        let glyph = self.infos[index].glyph().unpositioned();
                        let width = glyph.h_metrics().advance_width;
                        width / glyph.scale().x * unit as f32
                    };
                    text_width += width as u32;
                }

                let margin = (
                    ((t.rect[2] as i32 - text_width as i32) / 2).max(0) as u32,
                    (t.rect[3] - unit) / 2,
                );

                for i in 0..value.len() {
                    let x = rect[0] + margin.0 + x_offsets[i];
                    let y = rect[1] + margin.1;
                    for _ in 0..4 {
                        self.cell_rects.buffer.extend_from_slice(&[
                            x,
                            y,
                            unit as u32,
                            unit as u32,
                        ]);
                    }
                    count += 1;
                }
            }
        }

        self.cell_rects.update(glow::STATIC_DRAW);
        self.glyph_indices.update(glow::STATIC_DRAW);
        self.num_instances = count;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.obj
            .program()
            .set_uniform("viewport", rect(0, 0, width as i32, height as i32));
    }
}
