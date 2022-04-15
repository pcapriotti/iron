use rand::Rng;

pub type Value = u8;

/// State of the game.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct Game {
    width: usize,
    height: usize,
    tiles: Vec<Option<Value>>,
}

pub enum Direction {
    E,
    N,
    W,
    S,
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
        let value = if rng.gen_bool(0.1) { 2 } else { 1 };
        self.tiles[index] = Some(value);
    }

    pub fn all_tiles<'a>(
        &'a self,
    ) -> impl Iterator<Item = ((usize, usize), &'a Option<Value>)> + 'a {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, v)| ((i % self.width(), i / self.width()), v))
    }

    pub fn tiles<'a>(
        &'a self,
    ) -> impl Iterator<Item = ((usize, usize), Value)> + 'a {
        self.tiles.iter().enumerate().filter_map(|(i, v)| {
            v.map(|v| ((i % self.width(), i / self.width()), v))
        })
    }

    pub fn step(&mut self, dir: Direction) {
        let (width, height) = match dir {
            Direction::S | Direction::N => (self.width(), self.height()),
            Direction::E | Direction::W => (self.height(), self.width()),
        };

        let get: Box<dyn Fn(usize, usize) -> usize> = match dir {
            Direction::E => Box::new(|x, y| height - y - 1 + x * width),
            Direction::N => Box::new(|x, y| x + (height - y - 1) * width),
            Direction::W => Box::new(|x, y| y + x * width),
            Direction::S => Box::new(|x, y| x + y * width),
        };

        for x in 0..width {
            let mut y0 = 0;

            for y1 in 0..height {
                if let Some(v) = self.tiles[get(x, y1)] {
                    if y0 == y1 {
                        continue;
                    }
                    self.tiles[get(x, y1)] = None;
                    match self.tiles[get(x, y0)] {
                        None => {
                            self.tiles[get(x, y0)] = Some(v);
                        }
                        Some(w) if w == v => {
                            self.tiles[get(x, y0)] = Some(v + 1);
                            y0 += 1;
                        }
                        Some(_) => {
                            y0 += 1;
                            self.tiles[get(x, y0)] = Some(v);
                        }
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_south_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[0] = Some(0);
        let game2 = game.clone();
        game.step(Direction::S);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_east_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[3] = Some(2);
        let game2 = game.clone();
        game.step(Direction::E);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_north_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[13] = Some(2);
        let game2 = game.clone();
        game.step(Direction::N);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_west_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[0] = Some(2);
        let game2 = game.clone();
        game.step(Direction::W);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_south_simple() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(0);
        game.tiles[9] = Some(1);
        game.step(Direction::S);

        let mut game2 = Game::new(4, 4);
        game2.tiles[0] = Some(0);
        game2.tiles[1] = Some(1);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_east_simple() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(0);
        game.tiles[9] = Some(1);
        game.step(Direction::E);

        let mut game2 = Game::new(4, 4);
        game2.tiles[7] = Some(0);
        game2.tiles[11] = Some(1);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_north_simple() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(0);
        game.tiles[9] = Some(1);
        game.step(Direction::N);

        let mut game2 = Game::new(4, 4);
        game2.tiles[12] = Some(0);
        game2.tiles[13] = Some(1);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_west_simple() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(0);
        game.tiles[9] = Some(1);
        game.step(Direction::W);

        let mut game2 = Game::new(4, 4);
        game2.tiles[4] = Some(0);
        game2.tiles[8] = Some(1);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_merge() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(0);
        game.tiles[8] = Some(0);
        game.step(Direction::S);

        let mut game2 = Game::new(4, 4);
        game2.tiles[0] = Some(1);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_double_merge() {
        let mut game = Game::new(4, 4);
        game.tiles[0] = Some(1);
        game.tiles[4] = Some(1);
        game.tiles[8] = Some(3);
        game.tiles[12] = Some(3);
        game.step(Direction::S);

        let mut game2 = Game::new(4, 4);
        game2.tiles[0] = Some(2);
        game2.tiles[4] = Some(4);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_stuck() {
        let mut game = Game::new(4, 4);
        game.tiles[1] = Some(1);
        game.tiles[5] = Some(2);
        game.tiles[9] = Some(3);
        game.tiles[13] = Some(4);
        let game2 = game.clone();
        game.step(Direction::S);

        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_merge_in_place() {
        let mut game = Game::new(4, 4);
        game.tiles[1] = Some(1);
        game.tiles[5] = Some(1);
        game.tiles[9] = Some(2);
        game.tiles[13] = Some(2);
        game.step(Direction::S);

        let mut game2 = Game::new(4, 4);
        game2.tiles[1] = Some(2);
        game2.tiles[5] = Some(3);

        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_merge_in_place_equal() {
        let mut game = Game::new(4, 4);
        game.tiles[1] = Some(1);
        game.tiles[5] = Some(1);
        game.tiles[9] = Some(1);
        game.tiles[13] = Some(1);
        game.step(Direction::S);

        let mut game2 = Game::new(4, 4);
        game2.tiles[1] = Some(2);
        game2.tiles[5] = Some(2);

        assert_eq!(game, game2);
    }
}
