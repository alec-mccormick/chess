use derive_more::{From, Into, Deref};
use bevy::prelude::*;
use crate::prelude::*;



/// Components
///
#[derive(Debug, Copy, Clone)]
pub enum UnitType {
    Pawn,
}


#[derive(Debug, Copy, Clone, From, Into, Deref)]
pub struct UnitHealth(pub u32);

#[derive(Debug, Clone, From, Into, Deref)]
pub struct UnitTeam(pub String);



pub mod events {
    use crate::prelude::*;
    use super::{UnitHealth};

    #[derive(Debug, Copy, Clone)]
    pub struct PositionChanged(pub Position);

    #[derive(Debug, Copy, Clone)]
    pub struct HealthChanged(pub UnitHealth);
}


#[derive(Debug, Default)]
pub struct UnitComponentsPlugin;

impl Plugin for UnitComponentsPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(EventPlugin::<events::PositionChanged>::new())
            .add_plugin(EventPlugin::<events::HealthChanged>::new())
            .add_system(UnitComponentsPlugin::handle_position_changed.system());

    }
}

impl UnitComponentsPlugin {

    fn handle_position_changed(
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
}