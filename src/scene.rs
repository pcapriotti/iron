use crate::game::{Game, Move};
use crate::glyphs::Glyphs;
use crate::layout::Layout;
use crate::tiles::{Tile, Tiles};

pub struct Scene {
    tiles: Tiles,
    glyphs: Glyphs,
}

impl Scene {
    pub fn new(gl: &glow::Context, width: usize, height: usize) -> Scene {
        Scene {
            tiles: Tiles::new(gl, width * height * 3),
            glyphs: Glyphs::new(gl, width * height * 10),
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
        moves: &Vec<Move>,
        time: f32,
    ) {
        // compute base tile positions and colours
        let mut fg = game
            .all_tiles()
            .map(|(pos, value)| {
                value.map(|value| {
                    let colour = match value % 11 {
                        1 => [0.9, 0.8, 0.8],
                        2 => [0.84765625, 0.62890625, 0.203125],
                        3 => [0.765625, 0.41015625, 0.79296875],
                        4 => [0.015625, 0.42578125, 0.78125],
                        5 => [0.87890625, 0.39453125, 0.30078125],
                        6 => [0.52, 0.72265625, 0.09375],
                        7 => [0.359375, 0.70703125, 0.69921875],
                        8 => [0.6796875, 0.2421875, 0.38671875],
                        9 => [0.4375, 0.75390625, 0.69921875],
                        10 => [0.06640625, 0.515625, 0.65234375],
                        0 => [0.3828125, 0.5625, 0.78125],
                        _ => unreachable!(),
                    };

                    let rect = layout.rect(pos);
                    Tile {
                        pos,
                        value: Some(value),
                        colour,
                        rect,
                    }
                })
            })
            .collect::<Vec<_>>();

        for mv in moves.iter() {
            let src_point = (mv.src % layout.width, mv.src / layout.width);
            let dst_point = (mv.dst % layout.width, mv.dst / layout.width);

            let dx = ((dst_point.0 as f32 - src_point.0 as f32)
                * layout.unit as f32
                * time) as i32;
            let dy = ((dst_point.1 as f32 - src_point.1 as f32)
                * layout.unit as f32
                * time) as i32;

            if let Some(tile) = &mut fg[mv.src] {
                tile.rect[0] =
                    std::cmp::max(tile.rect[0] as i32 + dx, 0) as u32;
                tile.rect[1] =
                    std::cmp::max(tile.rect[1] as i32 + dy, 0) as u32;
            }
        }

        // collect all tiles
        let mut tiles = game
            .all_tiles()
            .map(|(pos, _)| Tile {
                pos,
                value: None,
                colour: [0.2, 0.2, 0.2],
                rect: layout.rect(pos),
            })
            .collect::<Vec<_>>();
        for t in fg {
            if let Some(t) = t {
                tiles.push(t);
            }
        }

        // update GPU state
        self.tiles.update(gl, layout, &tiles);
        self.glyphs.update(gl, layout, &tiles);
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
