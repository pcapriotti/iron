use crate::game::Game;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Animation<T> {
    start: Instant,
    duration: Duration,
    pub inner: T,
    pub result: Game,
}

pub fn default_duration_ms() -> u64 {
    160
}

const EASE_PARAMETER: f32 = 2.5;

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
        let t = (now - self.start).as_nanos() as f32 / self.duration.as_nanos() as f32;
        if t >= 0.0 && t <= 1.0 {
            let tp0 = t.powf(EASE_PARAMETER);
            let tp1 = (1.0 - t).powf(EASE_PARAMETER);
            tp0 / (tp0 + tp1)
        } else {
            t
        }
    }
}
