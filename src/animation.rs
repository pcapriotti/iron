use crate::game::Move;
use crate::layout::Layout;
use crate::tiles::Tile;
use std::time::{Duration, Instant};

pub struct Animation<T> {
    start: Instant,
    duration: Duration,
    pub inner: T,
}

pub const DEFAULT_DURATION: Duration = Duration::from_millis(100);

impl<T> Animation<T> {
    pub fn new(duration: Duration, inner: T) -> Self {
        Self {
            start: Instant::now(),
            duration,
            inner,
        }
    }

    pub fn time(&self) -> f32 {
        let now = Instant::now();
        (now - self.start).as_nanos() as f32 / self.duration.as_nanos() as f32
    }
}

pub trait Actuator<A> {
    fn actuate(&mut self, anim: &A, time: f32);

    fn finalise(&mut self, anim: &A) {
        self.actuate(anim, 1.0);
    }

    fn cancel(&mut self, _anim: &A) {}
}

pub struct MoveActuator<'a> {
    pub layout: &'a Layout,
    pub tiles: &'a mut [Option<Tile>],
}

impl<'a> Actuator<Vec<Move>> for MoveActuator<'a> {
    fn actuate(&mut self, moves: &Vec<Move>, time: f32) {
        for mv in moves.iter() {
            let src_point =
                (mv.src % self.layout.width, mv.src / self.layout.width);
            let dst_point =
                (mv.dst % self.layout.width, mv.dst / self.layout.width);

            let dx = ((dst_point.0 as f32 - src_point.0 as f32)
                * self.layout.unit as f32
                * (1.0 - time)) as i32;
            let dy = ((dst_point.1 as f32 - src_point.1 as f32)
                * self.layout.unit as f32
                * (1.0 - time)) as i32;

            if let Some(tile) = &mut self.tiles[mv.dst] {
                tile.rect[0] = (tile.rect[0] as i32 - dx) as u32;
                tile.rect[1] = (tile.rect[1] as i32 - dy) as u32;
            }
        }
    }
}
