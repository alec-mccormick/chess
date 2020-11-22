use bevy::prelude::*;
use crate::prelude::*;

use super::map;
use super::super::unit::{UnitStore, UnitCmd};

#[derive(Debug, Clone, Default)]
pub struct TileUIPlugin;

impl Plugin for TileUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(button_system.system())
        ;
    }
}



#[derive(PartialEq)]
enum InterfaceState {
    Default,
    UnitSelected(Entity)
}

struct Store {
    state: InterfaceState
}

impl Default for Store {
    fn default() -> Self {
        Store { state: InterfaceState::Default }
    }
}

fn button_system(
    mut store: Local<Store>,
    unit_store: Res<UnitStore>,
    entity_store: Res<EntityStore>,
    mut cmds: ResMut<Events<(ObjectId, UnitCmd)>>,
    mut interaction_query: Query<With<map::Tile, (Mutated<Interaction>, &Position)>>,
    mut object_id_query: Query<(&ObjectId)>,
) {
    for (interaction, position) in interaction_query.iter_mut() {

        match *interaction {
            Interaction::Clicked => {
                println!("Pressed! {:?}", position);

                match store.state {
                    InterfaceState::Default => {
                        if let Some(object_id) = unit_store.get_unit_by_position(position) {

                            if let Some(entity) = entity_store.get(object_id) {
                                store.state = InterfaceState::UnitSelected(*entity);
                            }
                        }
                    },
                    InterfaceState::UnitSelected(entity) => {
                        let (&object_id) = object_id_query.get(entity).unwrap();
                        cmds.send((object_id, UnitCmd::ExecuteAction(0, *position)));
                    }
                }
            }
            Interaction::Hovered => {
                println!("Hover {:?}", position);
            }
            Interaction::None => {
                println!("None {:?}", position);
            }
        }
    }
}