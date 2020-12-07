use bevy::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    fmt::Display
};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self { Id(uuid) }
    pub fn to_uuid(&self) -> &Uuid { &self.0 }
}


#[derive(Debug, Default)]
pub struct EntityIds {
    id_to_entity: HashMap<Id, Entity>,
    entity_to_id: HashMap<Entity, Id>,
}

impl EntityIds {
    pub fn get_id(&self, entity: &Entity) -> Option<&Id> {
        self.entity_to_id.get(entity)
    }

    pub fn get_entity(&self, id: &Id) -> Option<&Entity> {
        self.id_to_entity.get(id)
    }

    pub fn set(&mut self, id: Id, entity: Entity) {
        self.id_to_entity.insert(id.clone(), entity.clone());
        self.entity_to_id.insert(entity, id);
    }
}

pub fn entity_ids_system(
    mut entity_ids: ResMut<EntityIds>,
    query: Query<(Entity, Changed<Id>)>
) {
    for (entity, id) in query.iter() {
        entity_ids.set(*id, entity);
    }
}