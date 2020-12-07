use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, ops::Add};
use super::entity_map::EntityMap;


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

// ==========================================================================
// --- PositionMap
// ==========================================================================
pub struct PositionMap<T> {
    entity_map: EntityMap<Position>,
    phantom: PhantomData<T>,
}

impl<T> PositionMap<T> {
    fn default() -> Self {
        PositionMap::<T> {
            entity_map: EntityMap::<Position>::default(),
            phantom: PhantomData::<T>,
        }
    }
}

impl<T> PositionMap<T> {

    pub fn get(&self, pos: &Position) -> Option<&Entity> {
        self.entity_map.get(pos)
    }

    pub fn get_position(&self, entity: &Entity) -> Option<&Position> {
        self.entity_map.get_key(entity)
    }

    pub fn set(&mut self, pos: Position, entity: Entity) {
        self.entity_map.set(pos, entity)
    }

    pub fn remove(&mut self, pos: &Position) -> Option<()> {
        self.entity_map.remove(pos)
    }

    pub fn remove_entity(&mut self, entity: &Entity) -> Option<()> {
        self.entity_map.remove_entity(entity)
    }
}

// ==========================================================================
// --- Systems
// ==========================================================================
pub fn handle_position_changed_system<T>(
    mut position_map: ResMut<PositionMap<T>>,
    query: Query<With<T, (Entity, Changed<Position>)>>,
) where T: Send + Sync + 'static
{
    for (entity, position) in query.iter() {
        position_map.set(*position, entity);
    }
}

// TODO: HANDLE DESPAWNING

// ==========================================================================
// --- AddPositionMap
// ==========================================================================
pub trait AddPositionMap {
    fn add_position_map<T>(&mut self) -> &mut Self
        where T: Send + Sync + 'static;
}

impl AddPositionMap for AppBuilder {
    fn add_position_map<T>(&mut self) -> &mut Self
        where T: Send + Sync + 'static
    {
       self.add_resource(PositionMap::<T>::default())
           .add_system_to_stage(stage::POST_UPDATE, handle_position_changed_system::<T>.system())
    }
}