use std::ops::{Add, Deref};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position::new(x, y)
    }
}

impl From<&(i32, i32)> for Position {
    fn from((x, y): &(i32, i32)) -> Self {
        Position::new(*x, *y)
    }
}
