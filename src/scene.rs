use crate::config::Config;
use crate::game::{Game, Move, Value};
use crate::glyphs::Glyphs;
use crate::graphics::Quad;
use crate::layout::Layout;
use crate::tiles::{Tile, Tiles};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Scene {
    tiles: Tiles,
    glyphs: Glyphs,
    screen: Tiles,
}

impl Scene {
    pub fn new(gl: Rc<glow::Context>, config: &Config) -> Scene {
        let quad = Rc::new(RefCell::new(Quad::new(gl.clone())));
        let tiles = Tiles::new(gl.clone(), quad.clone(), config.tile_radius, 1.0);
        let glyphs = Glyphs::new(gl.clone(), quad.clone());
        let screen = Tiles::new(gl.clone(), quad.clone(), 0.0, 0.75);
        Scene {
            tiles,
            glyphs,
            screen,
        }
    }

    pub fn update(&mut self, layout: &Layout, game: &Game, moves: &Vec<Move>, time: f32) {
        // compute base tile positions and colours
        let mut fg = game
            .all_tiles()
            .map(|(pos, value)| {
                value.map(|value| {
                    let colour = match value % 12 {
                        0 => [0.12109375, 0.46875, 0.703125],
                        1 => [0.6484375, 0.8046875, 0.88671875],
                        2 => [0.6953125, 0.87109375, 0.5390625],
                        3 => [0.98046875, 0.6015625, 0.59765625],
                        4 => [0.88671875, 0.1015625, 0.109375],
                        5 => [0.19921875, 0.625, 0.171875],
                        6 => [0.98828125, 0.74609375, 0.43359375],
                        7 => [0.83203125, 0.7578125, 0.87109375],
                        8 => [0.99609375, 0.49609375, 0.0],
                        9 => [0.99609375, 0.99609375, 0.59765625],
                        10 => [0.4140625, 0.23828125, 0.6015625],
                        11 => [0.69140625, 0.34765625, 0.15625],
                        _ => unreachable!(),
                    };

                    let rect = layout.rect(pos);
                    (Tile { colour, rect }, value)
                })
            })
            .collect::<Vec<_>>();

        let mut merged: Vec<(Tile, Option<Value>)> = Vec::new();

        for mv in moves.iter() {
            let src_point = (mv.src % layout.width, mv.src / layout.width);
            let dst_point = (mv.dst % layout.width, mv.dst / layout.width);

            let dx = ((dst_point.0 as f32 - src_point.0 as f32) * layout.unit as f32 * time) as i32;
            let dy = ((dst_point.1 as f32 - src_point.1 as f32) * layout.unit as f32 * time) as i32;

            if let Some((tile, _)) = &mut fg[mv.src] {
                tile.rect[0] = std::cmp::max(tile.rect[0] as i32 + dx, 0) as u32;
                tile.rect[1] = std::cmp::max(tile.rect[1] as i32 + dy, 0) as u32;
            }
            if mv.merge {
                if let Some((t, v)) = fg[mv.src].take() {
                    merged.push((t, Some(v)));
                }
            }
        }

        // collect all tiles
        let mut tiles = game
            .all_tiles()
            .map(|(pos, _)| {
                (
                    Tile {
                        colour: [0.2, 0.2, 0.2],
                        rect: layout.rect(pos),
                    },
                    None,
                )
            })
            .collect::<Vec<_>>();
        for t in fg {
            if let Some((t, v)) = t {
                tiles.push((t, Some(v)));
            }
        }

        self.render_tiles(&tiles);
        self.render_tiles(&merged);

        // render screen
        if game.is_over() {
            self.screen.update(
                [Tile {
                    colour: [0.5, 0.5, 0.5],
                    rect: [
                        layout.origin.0,
                        layout.origin.1,
                        layout.size.0,
                        layout.size.1,
                    ],
                }]
                .iter(),
            );

            self.glyphs.update(
                [(
                    &[
                        layout.origin.0,
                        layout.origin.1,
                        layout.size.0,
                        layout.size.1,
                    ],
                    "Game over",
                )]
                .into_iter(),
            );
        }
    }

    fn render_tiles(&mut self, tiles: &[(Tile, Option<Value>)]) {
        self.tiles.update(tiles.iter().map(|(t, _)| t));
        let gtiles = tiles
            .iter()
            .filter_map(|(t, v)| v.map(|v| (&t.rect, format!("{}", (1 as u64) << v))));

        self.glyphs.update(gtiles);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.tiles.resize(width, height);
        self.glyphs.resize(width, height);
        self.screen.resize(width, height);
    }
}
