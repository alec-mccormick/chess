use bevy::prelude::*;
use crate::prelude::*;

use crate::core::{
    GameState, Tile,
    unit::{UnitStore, UnitCmd, Unit, Actions, Team, Health, is_action_valid}
};

#[derive(Debug, Clone, Copy)]
pub enum InputState {
    Default,
    UnitSelected(Entity)
}

pub fn handle_tile_interaction(
    mut input_state: ResMut<InputState>,
    game_state: Res<GameState>,
    unit_store: Res<UnitStore>,
    mut cmds: ResMut<Events<UnitCmd>>,
    mut interaction_query: Query<With<Tile, (Mutated<Interaction>, &Position)>>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) {
    for (interaction, position) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                match *input_state {
                    InputState::Default => {
                        if let Some(entity) = unit_store.get_unit(position) {
                            println!("-- unit selected: {:?}", entity);

                            let team = action_query
                                .get_component::<Team>(*entity)
                                .unwrap();

                            if team.eq(&game_state.active_team) {
                                println!("Unit for active team selected");
                                *input_state = InputState::UnitSelected(*entity);
                            } else {
                                println!("Inactive unit selected");
                            }
                        }
                    },
                    InputState::UnitSelected(entity) => {
                        let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
                        let action = actions.get(0).unwrap();

                        if is_action_valid(action, &entity, position, &unit_store, &action_query) {
                            println!("Execute action {:?}", entity);
                            cmds.send(UnitCmd::ExecuteAction(entity, 0, *position));
                        }

                        *input_state = InputState::Default;
                    }
                }
            }
            Interaction::Hovered => {
                println!("Hover {:?}", position);
            }
            Interaction::None => {
                // println!("None {:?}", position);
            }
        }
    }
}