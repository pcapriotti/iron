use crate::game::{Game, Move};
use crate::graphics::util::rect;
use crate::graphics::{
    GlyphCache, GlyphInfo, Instancing::*, Object, Quad, VertexBuffer,
};
use crate::layout::Layout;

pub struct Glyphs {
    obj: Object,

    cell_rects: VertexBuffer,
    glyph_indices: VertexBuffer,

    cache: GlyphCache,
    infos: Vec<GlyphInfo<'static>>,
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

    pub fn update(
        &mut self,
        gl: &glow::Context,
        layout: &Layout,
        game: &Game,
        moves: &[Move],
        time: f32,
    ) {
        let mut cell_rects: Vec<i32> = Vec::new();
        let mut glyph_indices: Vec<u32> = Vec::new();

        let unit = (layout.unit as f32 * 0.28) as u32;

        let mut count = 0;
        for ((x, y), value) in game.tiles() {
            let value = format!("{}", 1 << value);

            let mut x_offsets = Vec::new();
            let mut text_width = 0;
            for d in value.chars() {
                let index = self.cache.index_of(d);
                glyph_indices.push(index as u32);

                x_offsets.push(text_width);
                let width = {
                    let glyph = self.infos[index].glyph().unpositioned();
                    let width = glyph.h_metrics().advance_width;
                    width / glyph.scale().x * unit as f32
                };
                text_width += width as u32;
            }

            let margin =
                ((layout.unit - text_width) / 2, (layout.unit - unit) / 2);

            let (delta_x, delta_y) = if let Some(mv_index) =
                moves.iter().position(|m| m.dst == x + y * game.width())
            {
                let mv = &moves[mv_index];

                let src_point = (mv.src % game.width(), mv.src / game.width());
                let dst_point = (x, y);
                let delta_x = ((dst_point.0 as f32 - src_point.0 as f32)
                    * layout.unit as f32
                    * (1.0 - time)) as i32;
                let delta_y = ((dst_point.1 as f32 - src_point.1 as f32)
                    * layout.unit as f32
                    * (1.0 - time)) as i32;
                (delta_x, delta_y)
            } else {
                (0, 0)
            };

            for i in 0..value.len() {
                let x = (layout.origin.0
                    + x as u32 * layout.unit
                    + margin.0
                    + x_offsets[i]) as i32
                    - delta_x;
                let y = (layout.origin.1 + y as u32 * layout.unit + margin.1)
                    as i32
                    - delta_y;
                cell_rects.extend_from_slice(&[x, y, unit as i32, unit as i32]);
                count += 1;
            }
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
