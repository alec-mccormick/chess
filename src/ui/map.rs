use crate::prelude::*;
use bevy::{ecs::Command, prelude::*};
use bevy_prototype_lyon::prelude::*;


use crate::core::{
    map::TileStore,
    unit::{Actions, Health, Team, Unit},
    Map, Tile,
};
use log::info;
use std::cmp::Ordering;

use std::collections::BTreeSet;

use crate::render::{
    map::{TileMaterials, TileOverlayState},
    utils::HALF_TILE_RENDER_WIDTH_PX,
};

use super::input::InputState;
use std::ops::Deref;


/// ==========================================================================
/// Input State
/// ==========================================================================
pub fn handle_input_state_change(
    mut previous_state: Local<Option<InputState>>,
    input_state: ChangedRes<InputState>,
    unit_position_map: Res<PositionMap<Unit>>,
    tile_store: Res<TileStore>,
    mut tile_query: Query<(&Tile, &mut TileOverlayState)>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) {
    info!(
        "handle_input_state_change() {:?}, previous: {:?}",
        *input_state, *previous_state
    );

    match *input_state {
        InputState::UnitSelected(entity) => {
            info!("! unit selected");

            let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
            let action = actions.get(0).unwrap();

            for target in action.list_targets(&entity, &unit_position_map, &action_query) {
                info!("! Target {:?}", target);

                let tile_entity = tile_store.0.get(&target).unwrap();

                let (_, mut tile_overlay_state) = tile_query.get_mut(*tile_entity).unwrap();

                if *tile_overlay_state != TileOverlayState::Visible {
                    *tile_overlay_state = TileOverlayState::Visible;
                }
            }
        }
        InputState::Idle => {
            if let Some(InputState::UnitSelected(entity)) = *previous_state {
                let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
                let action = actions.get(0).unwrap();

                for target in action.list_targets(&entity, &unit_position_map, &action_query) {
                    info!("! Target {:?}", target);

                    let tile_entity = tile_store.0.get(&target).unwrap();

                    let (_, mut tile_overlay_state) = tile_query.get_mut(*tile_entity).unwrap();

                    if *tile_overlay_state != TileOverlayState::Invisible {
                        *tile_overlay_state = TileOverlayState::Invisible;
                    }
                }
            }
        }
    };

    *previous_state = Some(*input_state);
}
