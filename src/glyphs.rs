use anyhow::Result;
use glow::HasContext;
use rusttype::gpu_cache::{Cache, TextureCoords};
use rusttype::{point, Font, Scale};

const MAIN_FONT_ID: usize = 0;

pub struct Glyphs {
    font: Font<'static>,
    cache: Cache<'static>,
    scale: Scale,
}

impl Glyphs {
    pub fn new() -> Result<Self> {
        let data = std::fs::read("/usr/share/fonts/TTF/Hack-Regular.ttf")?;
        let font: Font<'static> =
            Font::try_from_vec(data).ok_or(anyhow::anyhow!("Error loading font"))?;
        let scale = Scale { x: 200.0, y: 200.0 };

        let mut cache = Cache::builder().dimensions(1024, 1024).build();
        for c in 'a'..'z' {
            let glyph = font
                .glyph(c as char)
                .scaled(scale)
                .positioned(point(0.0, 0.0));
            cache.queue_glyph(MAIN_FONT_ID, glyph);
        }
        Ok(Self { font, cache, scale })
    }

    pub fn upload_atlas(&mut self, gl: &glow::Context) -> Result<()> {
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

    pub fn rect_for(&self, c: char) -> Result<Option<TextureCoords>> {
        let glyph = self
            .font
            .glyph(c)
            .scaled(self.scale)
            .positioned(point(0.0, 0.0));
        let result = self.cache.rect_for(MAIN_FONT_ID, &glyph)?;
        Ok(result)
    }
}
