use bevy::prelude::*;
use std::{collections::HashMap, hash::Hash};


/// EntityMap is a data structure for looking up spawned entities by a key type K.
///
/// It is primarily used in game for finding entities by Id or Position.
#[derive(Debug)]
pub struct EntityMap<K> {
    key_to_entity: HashMap<K, Entity>,
    entity_to_key: HashMap<Entity, K>,
}

impl<K> Default for EntityMap<K> {
    fn default() -> Self {
        EntityMap::<K> {
            key_to_entity: HashMap::new(),
            entity_to_key: HashMap::new()
        }
    }
}

impl<K> EntityMap<K> where K: Eq + Hash + Clone {
    pub fn get(&self, key: &K) -> Option<&Entity> {
        self.key_to_entity.get(key)
    }

    pub fn get_key(&self, entity: &Entity) -> Option<&K> {
        self.entity_to_key.get(entity)
    }

    pub fn set(&mut self, key: K, entity: Entity) {
        self.set_inner(&entity);

        self.key_to_entity.insert(key.clone(), entity.clone());
        self.entity_to_key.insert(entity, key);
    }

    fn set_inner(&mut self, entity: &Entity) -> Option<()> {
        let existing_key = self.entity_to_key.get(entity)?;
        let existing_entity = self.key_to_entity.get(existing_key)?;

        if entity.eq(existing_entity) {
            self.key_to_entity.remove(existing_key);
        }

        Some(())
    }

    pub fn remove(&mut self, key: &K) -> Option<()> {
        let existing_entity = self.key_to_entity.get(key)?;

        if let Some(existing_key) = self.entity_to_key.get(existing_entity) {
            if key.eq(existing_key) {
                self.entity_to_key.remove(existing_entity);
            }
        }

        self.key_to_entity.remove(key);
        Some(())
    }


    pub fn remove_entity(&mut self, entity: &Entity) -> Option<()> {
        let existing_key = self.entity_to_key.get(entity)?;

        if let Some(existing_entity) = self.key_to_entity.get(existing_key) {
            if entity.eq(existing_entity) {
                self.key_to_entity.remove(existing_key);
            }
        }

        self.entity_to_key.remove(entity);
        Some(())
    }
}

// ==========================================================================
// --- Systems
// ==========================================================================
pub fn handle_key_changed_system<K>(
    mut entity_map: ResMut<EntityMap<K>>,
    query: Query<(Entity, Changed<K>)>
)
    where K: Hash + Eq + Clone + Send + Sync + 'static {
    for (entity, key) in query.iter() {
        entity_map.set(key.clone(), entity);
    }
}

// TODO: HANDLE DESPAWNING

/// An AppBuilder extension which initializes an EntityMap for a key type K and adds the
/// systems to handle component updates.
pub trait AddEntityMap {
    fn add_entity_map<K>(&mut self) -> &mut Self
        where K: Hash + Eq + Copy + Send + Sync + 'static;
}

impl AddEntityMap for AppBuilder {

    fn add_entity_map<K>(&mut self) -> &mut Self
        where K: Hash + Eq + Clone + Send + Sync + 'static
    {
        self.add_resource(EntityMap::<K>::default())
            .add_system_to_stage(stage::POST_UPDATE, handle_key_changed_system::<K>.system())
    }
}