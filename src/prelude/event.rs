use bevy::{app::Events, prelude::*};
use derive_more::{From, Into, Deref};
use std::marker::PhantomData;
// use std::collections::{HashMap};


use super::entity::{ObjectId, EntityStore};






/// Plugin to add in a message type.
///
/// This plugin registers events with both id types and adds in a system to map it from object_id -> Entity.
///
#[derive(Debug, Default, Copy, Clone)]
pub struct EventPlugin<T> {
    _marker: PhantomData<T>
}

impl<T> Plugin for EventPlugin<T>
    where T: Send + Sync + Copy + 'static {

    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<(ObjectId, T)>()
            .add_event::<(Entity, T)>()
            .add_system(EventPlugin::<T>::map_object_id_to_entity.system());
    }
}

impl<T> EventPlugin<T>
    where T: Send + Sync + Copy + 'static {

    pub fn new() -> Self {
        EventPlugin { _marker: Default::default() }
    }

    fn map_object_id_to_entity(
        mut reader: Local<EventReader<(ObjectId, T)>>,
        object_events: Res<Events<(ObjectId, T)>>,
        mut entity_events: ResMut<Events<(Entity, T)>>,
        store: Res<EntityStore>
    ) {
        for (id, event) in reader.iter(&object_events) {
            println!("received event!");

            if let Some(entity) = store.get(&*id) {
                println!("found entity: {:?}", entity);

                // let v = entity.0;
                entity_events.send((*entity, *event));
            }
        }
    }
}