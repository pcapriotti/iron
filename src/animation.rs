use crate::game::Move;
use std::time::{Duration, Instant};

pub struct Animation {
    start: Instant,
    duration: Duration,
}

impl Animation {
    pub fn new(duration: Duration) -> Self {
        Animation {
            start: Instant::now(),
            duration,
        }
    }

    pub fn time(&self) -> f32 {
        let now = Instant::now();
        (now - self.start).as_nanos() as f32 / self.duration.as_nanos() as f32
    }
}

pub struct MoveAnimation {
    pub animation: Animation,
    pub moves: Vec<Move>,
}
