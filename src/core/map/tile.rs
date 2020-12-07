use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;


#[derive(Debug, Bundle)]
pub struct TileComponents {
    pub tile: Tile,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tile {
    Black,
    White,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "Black".into(),
            Self::White => "White".into(),
        }
    }
}