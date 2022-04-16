use crate::game::{Game, Move};
use crate::glyphs::Glyphs;
use crate::layout::Layout;
use crate::tiles::Tiles;

pub struct Scene {
    tiles: Tiles,
    glyphs: Glyphs,
}

impl Scene {
    pub fn new(gl: &glow::Context) -> Scene {
        Scene {
            tiles: Tiles::new(gl),
            glyphs: Glyphs::new(gl),
        }
    }

    pub fn cleanup(&mut self, gl: &glow::Context) {
        self.tiles.cleanup(gl);
        self.glyphs.cleanup(gl);
    }

    pub fn update(
        &mut self,
        gl: &glow::Context,
        layout: &Layout,
        game: &Game,
        moves: &[Move],
        time: f32,
    ) {
        self.tiles.update(gl, layout, game, moves, time);
        self.glyphs.update(gl, layout, game, moves, time);
    }

    pub unsafe fn render(&self, gl: &glow::Context) {
        self.tiles.render(gl);
        self.glyphs.render(gl);
    }

    pub fn resize(&mut self, gl: &glow::Context, width: u32, height: u32) {
        self.tiles.resize(gl, width, height);
        self.glyphs.resize(gl, width, height);
    }
}
