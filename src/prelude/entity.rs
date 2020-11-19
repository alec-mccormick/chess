use bevy::{app::Events, prelude::*};
use derive_more::{From, Into, Deref};
use std::collections::{HashMap};

/// Structs/Types
///
#[derive(Debug, Default, Copy, Clone, From, Into, Deref, Hash, PartialEq, Eq)]
pub struct ObjectId(pub u32);



/// Entity Storage Object
/// Maintains a map of object id -> in game entity
///
#[derive(Debug, Default, Clone)]
pub struct EntityStore {
    map: HashMap<ObjectId, Entity>,
}


impl EntityStore {
    pub fn insert(&mut self, id: ObjectId, entity: Entity) {
        self.map.insert(id, entity);
    }

    pub fn remove(&mut self, id: &ObjectId) {
        self.map.remove(id);
    }

    pub fn get(&self, id: &ObjectId) -> Option<&Entity> {
        self.map.get(id)
    }
}


/// Entity Events
///
pub mod events {
    use super::{ObjectId};
    use bevy::prelude::Entity;

    #[derive(Debug, Copy, Clone)]
    pub struct EntitySpawned(pub ObjectId, pub Entity);

    #[derive(Debug, Copy, Clone)]
    pub struct EntityDespawned(pub ObjectId, pub Entity);
}


/// Bundle of entity storage + events + listeners
///
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(EntityStore::default())
            .add_event::<events::EntitySpawned>()
            .add_event::<events::EntityDespawned>()
            .add_system(EntityPlugin::handle_entity_events.system());
    }
}

impl EntityPlugin {
    pub fn handle_entity_events(
        mut spawned_reader: Local<EventReader<events::EntitySpawned>>,
        spawned_events: Res<Events<events::EntitySpawned>>,
        mut despawned_reader: Local<EventReader<events::EntityDespawned>>,
        despawned_events: Res<Events<events::EntityDespawned>>,
        mut entity_store: ResMut<EntityStore>
    ) {
        for &event in spawned_reader.iter(&spawned_events) {
            let (object_id, entity) = (event.0, event.1);
            entity_store.insert(object_id, entity);
        }

        for event in despawned_reader.iter(&despawned_events) {
            entity_store.remove(&event.0);
        }
    }
}

