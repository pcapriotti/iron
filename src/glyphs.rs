use crate::graphics::util::rect;
use crate::graphics::{GlyphCache, GlyphInfo, Object, Program, Quad, VertexArray, VertexBuffer};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Glyphs {
    obj: Object,
    quad: Rc<RefCell<Quad>>,

    cell_rects: VertexBuffer<u32>,
    glyph_indices: VertexBuffer<u32>,

    cache: GlyphCache,
    infos: Vec<GlyphInfo<'static>>,
}

impl Glyphs {
    pub fn new(gl: Rc<glow::Context>, quad: Rc<RefCell<Quad>>) -> Self {
        let mut vao = VertexArray::new(gl.clone());

        let program = Program::new(
            gl.clone(),
            include_bytes!("../shaders/glyph.v.glsl"),
            include_bytes!("../shaders/glyph.f.glsl"),
        );

        vao.add_buffer(quad.borrow().vbo());

        // cell rects
        let cell_rects = VertexBuffer::new(gl.clone(), 4);

        // glyph indices
        let glyph_indices = VertexBuffer::new(gl.clone(), 1);

        vao.add_buffer(cell_rects.to_ref());
        vao.add_buffer(glyph_indices.to_ref());

        let mut cache = GlyphCache::new(gl.clone(), 0);
        let (infos, texture) = cache.make_atlas();
        cache.upload_atlas(&texture.bind());

        let obj = Object::new(gl, vao, quad.borrow().ebo(), Some(texture), program);

        Self {
            obj,
            quad,
            cell_rects,
            glyph_indices,
            cache,
            infos,
        }
    }

    pub fn update<'a>(&mut self, texts: impl Iterator<Item = (&'a [u32; 4], impl AsRef<str>)>) {
        self.cell_rects.buffer.truncate(0);
        self.glyph_indices.buffer.truncate(0);
        let mut count = 0;

        for (rect, text) in texts {
            let text = text.as_ref();
            let scale = if text.len() <= 4 {
                0.4
            } else if text.len() <= 6 {
                0.28
            } else if text.len() <= 8 {
                0.21
            } else {
                0.15
            };
            let unit = (rect[2] as f32 * scale) as u32;

            // layout text
            let mut x_offsets = Vec::new();
            let mut text_width = 0;
            for d in text.chars() {
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
                ((rect[2] as i32 - text_width as i32) / 2).max(0) as u32,
                (rect[3] - unit) / 2,
            );

            for i in 0..text.len() {
                let x = rect[0] + margin.0 + x_offsets[i];
                let y = rect[1] + margin.1;
                for _ in 0..4 {
                    self.cell_rects
                        .buffer
                        .extend_from_slice(&[x, y, unit as u32, unit as u32]);
                }
                count += 1;
            }
        }

        self.cell_rects.update(glow::STATIC_DRAW);
        self.glyph_indices.update(glow::STATIC_DRAW);
        self.quad.borrow_mut().ensure(count);
        self.obj.render(count);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.obj
            .program()
            .set_uniform("viewport", rect(0, 0, width as i32, height as i32));
    }
}
