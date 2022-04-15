use rusttype::{Point, Rect};
use std::ops::Add;

pub fn rect<T>(x: T, y: T, w: T, h: T) -> Rect<T>
where
    T: Add<Output = T>,
    T: Copy,
{
    Rect {
        min: Point { x, y },
        max: Point { x: x + w, y: y + h },
    }
}
