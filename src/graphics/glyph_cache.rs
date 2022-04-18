use crate::graphics::{BoundTexture, ShaderStorageBuffer, Texture};
use glow::HasContext;
use rusttype::gpu_cache::Cache;
use rusttype::{point, Font, Point, PositionedGlyph, Rect, Scale};
use std::rc::Rc;

const MAIN_FONT_ID: usize = 0;

/// Per-glyph information sent to the GPU
#[derive(Debug)]
pub struct GlyphInfo<'a> {
    #[allow(dead_code)]
    glyph: PositionedGlyph<'a>,
    uv_rect: Rect<f32>,
    rect: Rect<f32>,
}

impl<'a> GlyphInfo<'a> {
    pub fn write_to(&self, out: &mut Vec<u8>) {
        fn write_rect<T>(rect: &Rect<T>, out: &mut Vec<u8>)
        where
            T: std::ops::Sub<Output = T>,
            T: Copy,
            T: bytemuck::Pod,
        {
            out.extend_from_slice(bytemuck::bytes_of(&rect.min.x));
            out.extend_from_slice(bytemuck::bytes_of(&rect.min.y));
            out.extend_from_slice(bytemuck::bytes_of(&rect.width()));
            out.extend_from_slice(bytemuck::bytes_of(&rect.height()));
        }

        write_rect(&self.uv_rect, out);
        write_rect(&self.rect, out);
    }

    pub fn glyph(&self) -> &PositionedGlyph<'a> {
        &self.glyph
    }
}

/// A cache of glyphs to be passed to the GPU.
pub struct GlyphCache {
    font: Font<'static>,
    cache: Cache<'static>,
    scale: Scale,
    buffer: ShaderStorageBuffer,
}

impl GlyphCache {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 1024;
    const SCALE: f32 = 100.0;

    pub fn new(gl: Rc<glow::Context>, index: u32) -> Self {
        let data = std::fs::read("/usr/share/fonts/TTF/Hack-Bold.ttf")
            .expect("Could not read font");
        let font: Font<'static> =
            Font::try_from_vec(data).expect("Error loading font");
        let scale = Scale {
            x: Self::SCALE,
            y: Self::SCALE,
        };

        let cache = Cache::builder()
            .dimensions(Self::WIDTH, Self::HEIGHT)
            .build();

        let buffer = ShaderStorageBuffer::new(gl.clone());
        buffer.bind(&gl, index);

        Self {
            font,
            cache,
            scale,
            buffer,
        }
    }

    pub fn upload_atlas(
        &mut self,
        gl: &glow::Context,
        _tex: &BoundTexture,
    ) -> () {
        self.cache
            .cache_queued(|rect, data| unsafe {
                gl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    rect.min.x as i32,
                    rect.min.y as i32,
                    rect.width() as i32,
                    rect.height() as i32,
                    glow::RED,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(data),
                );
            })
            .unwrap();
    }

    pub fn index_of(&self, c: char) -> usize {
        c as usize - 0x21
    }

    pub fn make_atlas(
        &mut self,
        gl: &glow::Context,
    ) -> (Vec<GlyphInfo<'static>>, Texture) {
        // queue all printable ASCII characters
        let glyphs = {
            let mut glyphs = Vec::with_capacity(128);
            for c in 0x21..0x7f as u8 {
                let glyph = self
                    .font
                    .glyph(c as char)
                    .scaled(self.scale)
                    .positioned(point(0.0, 0.0));
                self.cache.queue_glyph(MAIN_FONT_ID, glyph.clone());
                glyphs.push(glyph);
            }
            glyphs
        };

        let tex = Texture::new(gl, Self::WIDTH, Self::HEIGHT);
        self.upload_atlas(gl, &tex.bind(gl));

        unsafe { gl.bind_texture(glow::TEXTURE_2D, None) };

        // populate info array
        let infos = {
            let mut infos: Vec<GlyphInfo> = Vec::with_capacity(glyphs.len());
            for glyph in glyphs {
                let (uv_rect, rect) = self
                    .cache
                    .rect_for(MAIN_FONT_ID, &glyph)
                    .unwrap()
                    .expect("Missing glyph in the cache");

                let vmetrics = glyph.font().v_metrics(self.scale);

                // scale rect and reposition
                let rect = Rect {
                    min: Point {
                        x: rect.min.x as f32 / Self::SCALE,
                        y: (-rect.min.y as f32 - vmetrics.descent)
                            / Self::SCALE,
                    },
                    max: Point {
                        x: rect.max.x as f32 / Self::SCALE,
                        y: (-rect.max.y as f32 - vmetrics.descent)
                            / Self::SCALE,
                    },
                };

                infos.push(GlyphInfo {
                    glyph,
                    uv_rect,
                    rect,
                })
            }

            infos
        };

        {
            let mut data = Vec::new();
            for info in infos.iter() {
                info.write_to(&mut data);
            }
            self.buffer.set_data(gl, &data);
        }

        (infos, tex)
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.buffer.cleanup(gl);
    }
}
