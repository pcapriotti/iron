#[derive(Clone, Copy)]
pub struct V2<T> {
    pub x: T,
    pub y: T,
}

impl<T> V2<T> {
    pub fn new(x: T, y: T) -> Self {
        V2 { x, y }
    }
}
