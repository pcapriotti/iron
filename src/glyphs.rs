use crate::graphics::{BoundTexture, ShaderStorageBuffer, Texture};
use anyhow::Result;
use glow::HasContext;
use rusttype::gpu_cache::Cache;
use rusttype::{point, Font, Point, PositionedGlyph, Rect, Scale};

const MAIN_FONT_ID: usize = 0;

/// A cache of glyphs to be passed to the GPU.
pub struct Glyphs {
    font: Font<'static>,
    cache: Cache<'static>,
    scale: Scale,
}

/// Per-glyph information sent to the GPU
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
}

impl Glyphs {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 1024;
    const SCALE: f32 = 200.0;

    pub fn new() -> Result<Self> {
        let data = std::fs::read("/usr/share/fonts/TTF/Hack-Regular.ttf")?;
        let font: Font<'static> = Font::try_from_vec(data)
            .ok_or(anyhow::anyhow!("Error loading font"))?;
        let scale = Scale {
            x: Self::SCALE,
            y: Self::SCALE,
        };

        // TODO: delete
        let mut cache = Cache::builder()
            .dimensions(Self::WIDTH, Self::HEIGHT)
            .build();
        for c in 'a'..'z' {
            let glyph = font
                .glyph(c as char)
                .scaled(scale)
                .positioned(point(0.0, 0.0));
            cache.queue_glyph(MAIN_FONT_ID, glyph);
        }
        Ok(Self { font, cache, scale })
    }

    pub fn upload_atlas(
        &mut self,
        gl: &glow::Context,
        _tex: &BoundTexture,
    ) -> Result<()> {
        self.cache.cache_queued(|rect, data| unsafe {
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
        })?;
        Ok(())
    }

    pub fn make_atlas(
        &mut self,
        gl: &glow::Context,
        index: i32,
    ) -> Result<Texture> {
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

        let mut tex = Texture::new(gl, Self::WIDTH, Self::HEIGHT);
        self.upload_atlas(gl, &tex.bind(gl))?;

        unsafe { gl.bind_texture(glow::TEXTURE_2D, None) };

        // populate info array
        let infos = {
            let mut infos: Vec<GlyphInfo> = Vec::with_capacity(glyphs.len());
            for glyph in glyphs {
                let (uv_rect, rect) = self
                    .cache
                    .rect_for(MAIN_FONT_ID, &glyph)?
                    .ok_or(anyhow::anyhow!("Missing glyph in the cache"))?;

                let vmetrics = glyph.font().v_metrics(Scale {
                    x: Glyphs::SCALE,
                    y: Glyphs::SCALE,
                });

                // scale rect and reposition
                let mut rect = Rect {
                    min: Point {
                        x: rect.min.x as f32 / Glyphs::SCALE,
                        y: (-rect.min.y as f32 - vmetrics.descent)
                            / Glyphs::SCALE,
                    },
                    max: Point {
                        x: rect.max.x as f32 / Glyphs::SCALE,
                        y: (-rect.max.y as f32 - vmetrics.descent)
                            / Glyphs::SCALE,
                    },
                };
                let glyph_width = rect.width();
                rect.min.x = 0.5 - glyph_width / 2.0;
                rect.max.x = 0.5 + glyph_width / 2.0;

                infos.push(GlyphInfo {
                    glyph,
                    uv_rect,
                    rect,
                })
            }

            infos
        };

        {
            let mut ssbo = ShaderStorageBuffer::new(gl);
            let mut data = Vec::new();
            for info in infos {
                info.write_to(&mut data);
            }
            ssbo.set_data(gl, &data);
            ssbo.bind(gl, index);
        }

        Ok(tex)
    }
}
