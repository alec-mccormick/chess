mod types;
pub mod events;
pub mod action;

pub use types::*;


use crate::prelude::*;
use bevy::prelude::*;


#[derive(Debug, Default)]
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(UnitStore::default())
            .add_plugin(EventPlugin::<events::PositionChanged>::new())
            .add_plugin(EventPlugin::<events::HealthChanged>::new())
            .add_plugin(EventPlugin::<events::UnitCmd>::new())
            .add_system(UnitPlugin::handle_entity_position_changed.system())
            .add_system(UnitPlugin::handle_position_changed.system())
            .add_system(UnitPlugin::handle_entity_spawned.system());
    }
}

impl UnitPlugin {

    fn handle_entity_position_changed(
        mut reader: Local<EventReader<(Entity, events::PositionChanged)>>,
        events: Res<Events<(Entity, events::PositionChanged)>>,
        mut query: Query<(&mut Position)>,
    ) {
        for (entity, event) in reader.iter(&events) {
            println!("-- PRINT Event: {:?} {:?}", entity, event);

            let mut position = query.get_mut(*entity).unwrap();    //fixme
            position.x = event.0.x;
            position.y = event.0.y;
        }
    }

    fn handle_position_changed(
        mut reader: Local<EventReader<(ObjectId, events::PositionChanged)>>,
        events: Res<Events<(ObjectId, events::PositionChanged)>>,
        mut store: ResMut<UnitStore>
    ) {
        for (id, event) in reader.iter(&events) {
            store.set_position(*id, event.0);
        }
    }

    fn handle_entity_spawned(
        mut spawned_reader: Local<EventReader<entity::events::EntitySpawned>>,
        spawned_events: Res<Events<entity::events::EntitySpawned>>,
        mut store: ResMut<UnitStore>,
        query: Query<(&Position)>,
    ) {
        for event in spawned_reader.iter(&spawned_events) {
            let (object_id, entity) = (event.0, event.1);

            let &position = query.get(entity).unwrap();
            store.set_position(object_id, position);
        }
    }
}


