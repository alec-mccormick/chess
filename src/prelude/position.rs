use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, ops::Add};


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

// pub struct PositionMap<T> where T: Hash + Eq {
//     position_to_entity: HashMap<Position, Entity>,
//     entity_to_position: HashMap<Entity, Position>,
//     phantom: PhantomData<T>,
// }
//
// impl<T> PositionMap<T> {
//     pub fn new() -> Self {
//         unimplemented!()
//     }
//
// }
//
// pub fn add_position_map<T>(app: &mut AppBuilder) {
//     app
//         .add_resource(PositionMap::<T>::new())
//         .add_system(handle_position_changed::<T>.system())
//     ;
// }
//
// fn handle_position_changed<T>(
//     mut position_map: ResMut<PositionMap<T>>,
//     query: Query<With<T, (Entity, Changed<Position>)>>
// ) {
//
// }
