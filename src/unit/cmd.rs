use bevy::{app::Events, prelude::*};
use std::marker::PhantomData;
use crate::prelude::*;
use super::components::{self, UnitHealth, UnitType, UnitTeam};
use super::store::{UnitStore};
use super::action::{self, UnitActions};



/// Actual Events:
///
#[derive(Debug, Copy, Clone)]
pub enum UnitCmd {
    SetPosition(Position),
    SetHealth(UnitHealth),
    ExecuteAction(u16, Position)
}





#[derive(Debug, Default)]
pub struct UnitCmdPlugin;

impl Plugin for UnitCmdPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(EventPlugin::<UnitCmd>::new())
            .add_system(UnitCmdPlugin::handle_unit_command.system())
        ;
    }
}

impl UnitCmdPlugin {

    fn handle_unit_command(
        mut reader: Local<EventReader<(ObjectId, UnitCmd)>>,
        events: Res<Events<(ObjectId, UnitCmd)>>,
        mut pos_events: ResMut<Events<(ObjectId, components::events::PositionChanged)>>,
        mut health_events: ResMut<Events<(ObjectId, components::events::HealthChanged)>>,
        mut action_events: ResMut<Events<(ObjectId, action::UnitActionExecuted)>>,
    ) {
        for (id, cmd) in reader.iter(&events) {
            match cmd {
                UnitCmd::SetPosition(pos) => {
                    pos_events.send((id, components::events::PositionChanged(pos)))
                },
                UnitCmd::SetHealth(health) => {
                    health_events.sent((id, components::events::HealthChanged(health)))
                },
                UnitCmd::ExecuteAction(index, pos) => {
                    action_events.send((id, action::UnitActionExecuted(index, pos)))
                }
            };
        }
    }
}
