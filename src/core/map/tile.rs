use bevy::prelude::*;
use crate::prelude::*;


#[derive(Debug, Bundle)]
pub struct TileComponents {
    pub tile: Tile,
    pub position: Position,
}


#[derive(Debug)]
pub enum Tile {
    Black,
    White,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "Black".into(),
            Self::White => "White".into()
        }
    }
}