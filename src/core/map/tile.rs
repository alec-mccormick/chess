use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::derive::Display;


use std::collections::HashMap;


#[derive(Debug, Bundle)]
pub struct TileComponents {
    pub tile: Tile,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum Tile {
    Black,
    White,
}