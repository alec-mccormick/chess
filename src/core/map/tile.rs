use crate::prelude::*;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;


#[derive(Debug, Bundle)]
pub struct TileComponents {
    pub tile: Tile,
    pub position: Position,
    pub labels: Labels,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Default)]
pub struct TileStore(pub HashMap<Position, Entity>);

impl TileStore {
    pub fn handle_position_changed(
        mut store: ResMut<TileStore>,
        query: Query<With<Tile, (Entity, Changed<Position>)>>,
    ) {
        for (entity, position) in query.iter() {
            store.0.insert(*position, entity);
        }
    }
}
