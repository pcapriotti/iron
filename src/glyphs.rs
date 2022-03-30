use anyhow::Result;
use glow::HasContext;
use rusttype::gpu_cache::Cache;
use rusttype::{point, Font, Scale};

const MAIN_FONT_ID: usize = 0;

pub struct Glyphs {
    cache: Cache<'static>,
}

impl Glyphs {
    pub fn new() -> Result<Self> {
        let data = std::fs::read("/usr/share/fonts/TTF/Hack-Regular.ttf")?;
        let font = Font::try_from_vec(data).ok_or(anyhow::anyhow!("Error loading font"))?;
        let scale = Scale { x: 20.0, y: 20.0 };

        let mut cache = Cache::builder().dimensions(512, 512).build();
        for c in "hello world".chars() {
            let glyph = font.glyph(c).scaled(scale).positioned(point(0.0, 0.0));
            cache.queue_glyph(MAIN_FONT_ID, glyph);
        }
        Ok(Self { cache })
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
}
