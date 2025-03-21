use rand::Rng;

pub type Value = u8;

/// State of the game.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct Game {
    width: usize,
    height: usize,
    pub tiles: Vec<Option<Value>>,
}

pub enum Direction {
    E,
    N,
    W,
    S,
}

#[derive(PartialEq, Debug)]
pub struct Move {
    pub src: usize,
    pub dst: usize,
    pub merge: bool,
}

impl Move {
    fn new(src: usize, dst: usize, merge: bool) -> Move {
        Move { src, dst, merge }
    }
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

    pub fn random_tile(&mut self) -> (usize, Value) {
        let empty_indices = (0..self.tiles.len())
            .filter(|i| self.tiles[*i].is_none())
            .collect::<Vec<_>>();
        let mut rng = rand::rng();
        let index = empty_indices[rng.random_range(0..empty_indices.len())];
        let value = if rng.random_bool(0.1) { 2 } else { 1 };
        (index, value)
    }

    pub fn add_random_tile(&mut self) {
        let (index, value) = self.random_tile();
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

    pub fn is_over(&self) -> bool {
        for (i, &value) in self.tiles.iter().enumerate() {
            if value.is_some() {
                if i % self.width() != 0 && self.tiles[i - 1] == value {
                    return false;
                }
                if i >= self.width() && self.tiles[i - self.width()] == value {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    pub fn step(&mut self, dir: Direction) -> Vec<Move> {
        let mut moves = Vec::new();

        let (width, height) = match dir {
            Direction::S | Direction::N => (self.width(), self.height()),
            Direction::E | Direction::W => (self.height(), self.width()),
        };

        let get: Box<dyn Fn(usize, usize) -> usize> = match dir {
            Direction::E => Box::new(|x, y| height - y - 1 + x * self.width),
            Direction::N => Box::new(|x, y| x + (height - y - 1) * self.width),
            Direction::W => Box::new(|x, y| y + x * self.width),
            Direction::S => Box::new(|x, y| x + y * self.width),
        };

        for x in 0..width {
            let mut y0 = 0;

            for y1 in 0..height {
                let i1 = get(x, y1);
                if let Some(v) = self.tiles[i1] {
                    if y0 == y1 {
                        continue;
                    }
                    let i0 = get(x, y0);
                    match self.tiles[i0] {
                        None => {
                            self.tiles[i1] = None;
                            self.tiles[i0] = Some(v);
                            moves.push(Move::new(i1, i0, false));
                        }
                        Some(w) if w == v => {
                            self.tiles[i1] = None;
                            self.tiles[i0] = Some(v + 1);
                            moves.push(Move::new(i1, i0, true));
                            y0 += 1;
                        }
                        Some(_) => {
                            y0 += 1;
                            let i0 = get(x, y0);
                            if i0 != i1 {
                                self.tiles[i1] = None;
                                self.tiles[i0] = Some(v);
                                moves.push(Move::new(i1, i0, false));
                            }
                        }
                    };
                }
            }
        }

        moves
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
        assert!(game.step(Direction::S).is_empty());
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_east_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[3] = Some(2);
        let game2 = game.clone();
        assert!(game.step(Direction::E).is_empty());
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_north_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[13] = Some(2);
        let game2 = game.clone();
        assert!(game.step(Direction::N).is_empty());
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_west_trivial() {
        let mut game = Game::new(4, 4);
        game.tiles[0] = Some(2);
        let game2 = game.clone();
        assert!(game.step(Direction::W).is_empty());
        assert_eq!(game, game2);
    }

    #[test]
    fn test_step_south_simple() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(0);
        game.tiles[9] = Some(1);
        assert_eq!(
            vec![Move::new(4, 0, false), Move::new(9, 1, false)],
            game.step(Direction::S)
        );

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
        assert_eq!(
            vec![Move::new(4, 7, false), Move::new(9, 11, false)],
            game.step(Direction::E)
        );

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
        assert_eq!(
            vec![Move::new(4, 12, false), Move::new(9, 13, false)],
            game.step(Direction::N)
        );

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
        assert_eq!(vec![Move::new(9, 8, false)], game.step(Direction::W));

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
        game.tiles[5] = Some(3);
        game.tiles[9] = Some(5);
        game.tiles[13] = Some(7);
        let game2 = game.clone();
        let moves = game.step(Direction::S);
        assert!(moves.is_empty());
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
        assert_eq!(
            vec![
                Move::new(5, 1, true),
                Move::new(9, 5, false),
                Move::new(13, 5, true)
            ],
            game.step(Direction::S)
        );

        let mut game2 = Game::new(4, 4);
        game2.tiles[1] = Some(2);
        game2.tiles[5] = Some(2);

        assert_eq!(game, game2);
    }

    #[test]
    fn test_merge_far() {
        let mut game = Game::new(4, 4);
        game.tiles[4] = Some(1);
        game.tiles[5] = Some(1);
        assert_eq!(
            vec![Move::new(5, 7, false), Move::new(4, 7, true)],
            game.step(Direction::E)
        );

        let mut game2 = Game::new(4, 4);
        game2.tiles[7] = Some(2);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_nonsquare_move() {
        let mut game = Game::new(5, 3);
        game.tiles[1] = Some(1);
        assert_eq!(vec![Move::new(1, 4, false)], game.step(Direction::E));

        let mut game2 = Game::new(5, 3);
        game2.tiles[4] = Some(1);
        assert_eq!(game, game2);
    }

    #[test]
    fn test_game_not_over() {
        let mut game = Game::new(4, 4);

        for i in 0..16 {
            assert!(!game.is_over());
            game.tiles[i] = Some(i as u8);
        }

        assert!(game.is_over());
    }

    #[test]
    fn test_gameover() {
        let mut game = Game::new(4, 4);
        game.tiles[0] = Some(1);
        game.tiles[1] = Some(3);
        game.tiles[2] = Some(5);
        game.tiles[3] = Some(2);
        game.tiles[4] = Some(2);
        game.tiles[5] = Some(6);
        game.tiles[6] = Some(8);
        game.tiles[7] = Some(9);
        game.tiles[8] = Some(3);
        game.tiles[9] = Some(4);
        game.tiles[10] = Some(5);
        game.tiles[11] = Some(11);
        game.tiles[12] = Some(1);
        game.tiles[13] = Some(3);
        game.tiles[14] = Some(9);
        game.tiles[15] = Some(2);

        assert!(game.is_over());
    }
}
