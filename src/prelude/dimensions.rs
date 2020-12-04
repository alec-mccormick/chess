#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

impl Dimensions {
    pub fn new(width: i32, height: i32) -> Self {
        Dimensions { width, height }
    }
}
