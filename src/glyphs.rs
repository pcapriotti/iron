use anyhow::Result;
use glow::HasContext;
use rusttype::gpu_cache::{Cache, TextureCoords};
use rusttype::{point, Font, PositionedGlyph, Rect, Scale};

const MAIN_FONT_ID: usize = 0;

pub struct Glyphs {
    font: Font<'static>,
    cache: Cache<'static>,
    scale: Scale,
}

/// Per-glyph information sent to the GPU
pub struct GlyphInfo<'a> {
    glyph: PositionedGlyph<'a>,
    coords: TextureCoords,
}

/// A glyph cache texture, together with a shader buffer object.
///
/// The buffer contains information for each glyph in the texture. The elements of the buffer are
/// ordered by character.
pub struct Atlas {}

impl<'a> GlyphInfo<'a> {
    pub fn write_to(&self, out: &mut Vec<u8>) {
        let (uv_rect, rect) = self.coords;

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

        write_rect(&uv_rect, out);
        write_rect(&rect, out);
    }
}

impl Glyphs {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 1024;

    pub fn new() -> Result<Self> {
        let data = std::fs::read("/usr/share/fonts/TTF/Hack-Regular.ttf")?;
        let font: Font<'static> = Font::try_from_vec(data)
            .ok_or(anyhow::anyhow!("Error loading font"))?;
        let scale = Scale { x: 200.0, y: 200.0 };

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

    pub fn make_atlas(
        &mut self,
        gl: &glow::Context,
    ) -> Result<(glow::NativeTexture, glow::NativeBuffer)> {
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

        // create texture
        let tex = unsafe {
            let tex = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                Self::WIDTH as i32,
                Self::HEIGHT as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(&vec![0xff; (Self::WIDTH * Self::HEIGHT) as usize]),
            );
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            tex
        };

        self.upload_atlas(&gl)?;

        unsafe { gl.bind_texture(glow::TEXTURE_2D, None) };

        // populate info array
        let infos = {
            let mut infos: Vec<GlyphInfo> = Vec::with_capacity(glyphs.len());
            for glyph in glyphs {
                let coords = self
                    .cache
                    .rect_for(MAIN_FONT_ID, &glyph)?
                    .ok_or(anyhow::anyhow!("Missing glyph in the cache"))?;
                infos.push(GlyphInfo { glyph, coords })
            }

            infos
        };

        // create shader storage buffer
        let ssbo = unsafe {
            let ssbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(ssbo));
            let data = {
                let mut data = Vec::new();
                for info in infos {
                    info.write_to(&mut data);
                }
                data
            };
            gl.buffer_data_u8_slice(
                glow::SHADER_STORAGE_BUFFER,
                &data,
                glow::STATIC_DRAW,
            );
            ssbo
        };

        Ok((tex, ssbo))
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
