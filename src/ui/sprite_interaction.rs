use bevy::prelude::*;
use crate::prelude::*;

use log::{trace};

use crate::core::Tile;
use crate::render::utils::{HALF_TILE_RENDER_WIDTH_PX, HALF_TILE_RENDER_HEIGHT_PX};


#[derive(Default)]
pub struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
    hovered_entity: Option<Entity>,
}


pub fn sprite_interaction_system(
    mut state: Local<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    touches_input: Res<Touches>,
    window: Res<WindowDescriptor>,
    mut query: Query<(Entity, &Tile, &GlobalTransform, &mut Interaction)>
) {
    let mut cursor_changed = false;

    if let Some(cursor_moved) = state.cursor_moved_event_reader.latest(&cursor_moved_events) {
        state.cursor_position = cursor_moved.position;
        cursor_changed = true;
        // trace!("cursor position: {:?}", cursor_moved.position);
    }
    if let Some(touch) = touches_input.get_pressed(0) {
        state.cursor_position = touch.position;
        cursor_changed = true;
    }

    let mouse_clicked = mouse_button_input.just_released(MouseButton::Left)
        || touches_input.just_released(0);

    if !cursor_changed && !mouse_clicked {
        return;
    }

    let center = Vec2::new(window.width as f32 / 2.0, window.height as f32 / 2.0);
    let cursor_position = state.cursor_position - center;

    let tile_size = Vec2::new(HALF_TILE_RENDER_WIDTH_PX as f32, HALF_TILE_RENDER_HEIGHT_PX as f32);

    let mut potential_tiles = query
        .iter_mut()
        .filter_map(|(entity, tile, global_transform, interaction)| {
            let position = global_transform.translation.truncate();
            let scaled_tile_size = tile_size * global_transform.scale.truncate();

            let btm_left = position - scaled_tile_size;
            let top_right = position + scaled_tile_size;

            if cursor_position >= btm_left && cursor_position <= top_right {
                return Some((entity, tile, position, interaction, btm_left, top_right));
            }

            None
        })
        .collect::<Vec<_>>();

    potential_tiles.sort_by(|(_, _, _, _, _, a), (_, _, _, _, _, b)| {
        a.y().partial_cmp(&b.y()).unwrap()
    });

    let mut hovered_entity = None;

    for (entity, _tile, position, mut interaction, btm_left, top_right) in potential_tiles {
        let is_valid = if cursor_position.x() < position.x() {
            let a = Vec2::new(btm_left.x(), position.y());

            if cursor_position.y() > position.y() {
                let b = Vec2::new(position.x(), top_right.y());
                !is_above_line(cursor_position, a, b)
            } else {
                let b = Vec2::new(position.x(), btm_left.y());
                is_above_line(cursor_position, a, b)
            }
        } else {
            let b = Vec2::new(top_right.x(), position.y());

            if cursor_position.y() > position.y() {
                let a = Vec2::new(position.x(), top_right.y());
                !is_above_line(cursor_position, a, b)
            } else {
                let a = Vec2::new(position.x(), btm_left.y());
                is_above_line(cursor_position, a, b)
            }
        };

        if is_valid {
            if mouse_clicked {
                if *interaction != Interaction::Clicked {
                    *interaction = Interaction::Clicked;
                }
            } else if *interaction != Interaction::Hovered {
                *interaction = Interaction::Hovered;
            }

            hovered_entity = Some(entity);

            break;
        }
    }

    if let Some(new_hovered_entity) = hovered_entity {
        if let Some(old_hovered_entity) = state.hovered_entity {
            if new_hovered_entity != old_hovered_entity {
                if let Ok(mut interaction) = query.get_component_mut::<Interaction>(old_hovered_entity) {
                    if *interaction == Interaction::Hovered {
                        *interaction = Interaction::None;
                    }
                }
                state.hovered_entity = None;
            }
        }

        state.hovered_entity = hovered_entity;
    }
}


fn is_above_line(point: Vec2, a: Vec2, b: Vec2) -> bool {
    let slope = (b.y() - a.y()) / (b.x() - a.x());

    return point.y() >= a.y() + slope * (point.x() - a.x());
}