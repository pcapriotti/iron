use crate::game::{Game, Move};
use crate::layout::Layout;
use crate::tiles::Tile;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Animation<T> {
    start: Instant,
    duration: Duration,
    pub inner: T,
    pub result: Game,
}

pub const DEFAULT_DURATION: Duration = Duration::from_millis(2000);

impl<T> Animation<T> {
    pub fn new(duration: Duration, inner: T, result: Game) -> Self {
        Self {
            start: Instant::now(),
            duration,
            inner,
            result,
        }
    }

    pub fn time(&self) -> f32 {
        let now = Instant::now();
        (now - self.start).as_nanos() as f32 / self.duration.as_nanos() as f32
    }
}

pub struct Seq<T, U> {
    pub a: T,
    pub b: U,
    pub alpha: f32,
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
                * time) as i32;
            let dy = ((dst_point.1 as f32 - src_point.1 as f32)
                * self.layout.unit as f32
                * time) as i32;

            if let Some(tile) = &mut self.tiles[mv.src] {
                tile.rect[0] =
                    std::cmp::max(tile.rect[0] as i32 + dx, 0) as u32;
                tile.rect[1] =
                    std::cmp::max(tile.rect[1] as i32 + dy, 0) as u32;
            }
        }
    }
}

pub struct SeqActuator<A, B> {
    a: A,
    b: B,
}

impl<A, B, T, U> Actuator<Seq<T, U>> for SeqActuator<A, B>
where
    A: Actuator<T>,
    B: Actuator<U>,
{
    fn actuate(&mut self, seq: &Seq<T, U>, time: f32) {
        if time <= seq.alpha {
            self.a.actuate(&seq.a, time / seq.alpha);
        } else {
            self.b
                .actuate(&seq.b, (time - seq.alpha) / (1.0 - seq.alpha));
        }
    }
}
