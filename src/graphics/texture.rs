use glow::HasContext;
use std::rc::Rc;

#[derive(Clone)]
pub struct Texture {
    gl: Rc<glow::Context>,
    inner: glow::NativeTexture,
}

impl Texture {
    pub fn new(gl: Rc<glow::Context>, width: u32, height: u32) -> Self {
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
                width as i32,
                height as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(&vec![0xff; (width * height) as usize])),
            );
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            tex
        };
        Self { gl, inner: tex }
    }

    pub fn bind<'a>(&'a self) -> BoundTexture<'a> {
        BoundTexture::new(&self.gl, &self)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.delete_texture(self.inner) };
    }
}

pub struct BoundTexture<'a> {
    pub inner: &'a Texture,
    gl: &'a glow::Context,
}

impl<'a> BoundTexture<'a> {
    fn new(gl: &'a glow::Context, texture: &'a Texture) -> Self {
        unsafe { gl.bind_texture(glow::TEXTURE_2D, Some(texture.inner)) };
        Self { inner: texture, gl }
    }
}

impl Drop for BoundTexture<'_> {
    fn drop(&mut self) {
        unsafe { self.gl.bind_texture(glow::TEXTURE_2D, None) };
    }
}
