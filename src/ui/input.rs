use bevy::prelude::*;
use bevy_networking::{NetworkDelivery, NetworkResource};
use log::info;

use crate::{
    core::{
        unit::{is_action_valid, Actions, Health, Team, Unit, UnitCmd},
        GameState, Tile,
    },
    prelude::*,
};

#[derive(Debug, Clone, Copy)]
pub enum InputState {
    Idle,
    UnitSelected(Entity),
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Idle
    }
}

pub fn handle_tile_interaction(
    mut input_state: ResMut<InputState>,
    game_state: Res<GameState>,
    unit_position_map: Res<PositionMap<Unit>>,
    mut cmds: ResMut<Events<UnitCmd>>,
    mut interaction_query: Query<With<Tile, (Mutated<Interaction>, &Position)>>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) {
    for (interaction, position) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => match *input_state {
                InputState::Idle => {
                    if let Some(entity) = unit_position_map.get(position) {
                        info!("-- unit selected: {:?}", entity);

                        let team = action_query.get_component::<Team>(*entity).unwrap();

                        if team.eq(&game_state.active_team) && team.eq(&game_state.local_player_info.team) {
                            info!("Unit for active team selected");
                            *input_state = InputState::UnitSelected(*entity);
                        } else {
                            info!("Inactive unit selected");
                        }
                    }
                }
                InputState::UnitSelected(entity) => {
                    let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
                    let action = actions.get(0).unwrap();

                    if is_action_valid(action, &entity, position, &unit_position_map, &action_query) {
                        info!("Execute action {:?}", entity);
                        cmds.send(UnitCmd::ExecuteAction(entity, 0, *position));
                    }

                    *input_state = InputState::Idle;
                }
            },
            Interaction::Hovered => {
                info!("Hover {:?}", position);
            }
            Interaction::None => {
                // println!("None {:?}", position);
            }
        }
    }
}
