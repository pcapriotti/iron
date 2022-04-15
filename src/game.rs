pub type Value = u8;

/// State of the game.
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
}
