use crate::game::Game;
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
