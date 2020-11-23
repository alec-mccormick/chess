use bevy::prelude::*;
use crate::prelude::*;

use crate::core::{map::Tile, unit::{UnitStore, UnitCmd}};




enum InterfaceState {
    Default,
    UnitSelected(Entity)
}


pub struct Store {
    state: InterfaceState
}


impl Default for Store {
    fn default() -> Self {
        Store { state: InterfaceState::Default }
    }
}

pub fn append_interaction_to_tile (
    mut commands: Commands,
    query: Query<With<Tile, Without<Interaction, (Entity)>>>
) {
    for (entity) in query.iter() {
        commands.insert_one(entity, Interaction::default());
    }
}

pub fn tile_interface_system(
    mut store: Local<Store>,
    unit_store: Res<UnitStore>,
    mut cmds: ResMut<Events<(Entity, UnitCmd)>>,
    mut interaction_query: Query<With<Tile, (Mutated<Interaction>, &Position)>>,
) {
    for (interaction, position) in interaction_query.iter_mut() {

        match *interaction {
            Interaction::Clicked => {
                // println!("Pressed! {:?}", position);

                match store.state {
                    InterfaceState::Default => {
                        if let Some(entity) = unit_store.get_unit(position) {
                            // println!("Unit selected!");
                            store.state = InterfaceState::UnitSelected(*entity);
                        }
                    },
                    InterfaceState::UnitSelected(entity) => {
                        // println!("execute action!");
                        cmds.send((entity, UnitCmd::ExecuteAction(0, *position)));
                    }
                }
            }
            Interaction::Hovered => {
                // println!("Hover {:?}", position);
            }
            Interaction::None => {
                // println!("None {:?}", position);
            }
        }
    }
}