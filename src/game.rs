use rand::Rng;

pub type Value = u8;

/// State of the game.
#[allow(dead_code)]
pub struct Game {
    width: usize,
    height: usize,
    tiles: Vec<Option<Value>>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();
        tiles.resize(width * height, None);
        Game {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn add_random_tile(&mut self) {
        let empty_indices = (0..self.tiles.len())
            .filter(|i| self.tiles[*i].is_none())
            .collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        let index = empty_indices[rng.gen_range(0..empty_indices.len())];
        let value = if rng.gen() { 0 } else { 1 };
        self.tiles[index] = Some(value);
    }

    pub fn tiles<'a>(&'a self) -> impl Iterator<Item = (usize, Value)> + 'a {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|v| (i, v)))
    }
}
