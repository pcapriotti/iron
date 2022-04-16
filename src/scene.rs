use crate::game::{Game, Move};
use crate::glyphs::Glyphs;
use crate::layout::Layout;
use crate::tiles::{Tile, Tiles};

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
        moves: &Vec<Move>,
        time: f32,
    ) {
        // compute base tile positions and colours
        let mut fg = game
            .all_tiles()
            .map(|(pos, value)| {
                value.map(|value| {
                    let colour = match value % 10 {
                        1 => [0.9453125, 0.6171875, 0.296875],
                        2 => [0.94140625, 0.765625, 0.32421875],
                        3 => [0.93359375, 0.9140625, 0.3515625],
                        4 => [0.72265625, 0.90234375, 0.41015625],
                        5 => [0.51171875, 0.88671875, 0.46484375],
                        6 => [0.0859375, 0.85546875, 0.57421875],
                        7 => [0.05078125, 0.69921875, 0.6171875],
                        8 => [0.015625, 0.54296875, 0.65625],
                        9 => [0.171875, 0.41015625, 0.6015625],
                        0 => [0.328125, 0.27734375, 0.546875],
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
