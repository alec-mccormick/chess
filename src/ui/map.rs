use bevy::prelude::*;
use crate::prelude::*;
use bevy::ecs::Command;
use bevy_prototype_lyon::prelude::*;


use crate::core::{Tile, Map, map::TileStore, unit::{Actions, Unit, Team, Health, UnitStore}};
use log::{trace};
use std::cmp::Ordering;

use std::collections::BTreeSet;

use crate::render::utils::HALF_TILE_RENDER_WIDTH_PX;
use crate::render::map::{TileMaterials, TileOverlayState};

use super::input::InputState;
use std::ops::Deref;

pub struct MapUIPlugin;

impl Plugin for MapUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(MapUIStore::default())
            .add_system(handle_map_spawned.system())
            .add_system(handle_tile_spawned.system())
            // .add_system(tile_interaction_system.system())
        ;
    }
}

/// ==========================================================================
/// Input State
/// ==========================================================================
pub fn handle_input_state_change(
    mut previous_state: Local<Option<InputState>>,
    input_state: ChangedRes<InputState>,
    unit_store: Res<UnitStore>,
    tile_store: Res<TileStore>,
    mut tile_query: Query<(&Tile, &mut TileOverlayState)>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) {
    println!("!!!handle_input_state_change() {:?}, previous: {:?}", *input_state, *previous_state);

    match *input_state {
        InputState::UnitSelected(entity) => {
            println!("! unit selected");

            let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
            let action = actions.get(0).unwrap();

            for target in action.list_targets(&entity, &unit_store, &action_query) {
                println!("! Target {:?}", target);

                let tile_entity = tile_store.0.get(&target).unwrap();

                let (_, mut tile_overlay_state) = tile_query.get_mut(*tile_entity).unwrap();

                if *tile_overlay_state != TileOverlayState::Visible {
                    *tile_overlay_state = TileOverlayState::Visible;
                }
            }
        }
        InputState::Default => {

            if let Some(InputState::UnitSelected(entity)) = *previous_state {

                let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
                let action = actions.get(0).unwrap();

                for target in action.list_targets(&entity, &unit_store, &action_query) {
                    println!("! Target {:?}", target);

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


/// ==========================================================================
/// Map UI
/// ==========================================================================
#[derive(Default)]
struct MapUIStore {
    columns: Vec<Entity>,
}


struct MapUIColumns(Vec<Entity>);

struct MapUIColumn {
    items: Vec<MapUIColumnItem>,
    left: bool
}

impl MapUIColumn {
    fn get_x_bounds(&self, transform: &GlobalTransform) -> (f32, f32) {
        let x = transform.translation.x();
        let scaled_width = transform.scale.x() * (HALF_TILE_RENDER_WIDTH_PX as f32);

        return if self.left {
            (x - scaled_width, x)
        } else {
            (x, x + scaled_width)
        }
    }

    fn contains_point(&self, point: &Vec2, transform: &GlobalTransform) -> bool {
        let (left, right) = self.get_x_bounds(transform);

        point.x() >= left && point.x() <= right
    }

    fn add(&mut self, item: MapUIColumnItem) {
        self.items.push(item);

        self.items.sort_by(|a, b| {
            let value = a.position.x + a.position.y;
            let other_value = b.position.x + b.position.y;
            value.cmp(&other_value)
        });
    }
}


struct MapUIColumnItem {
    entity: Entity,
    position: Position,
}

impl MapUIColumnItem {
    fn new (entity: Entity, position: Position) -> Self {
        MapUIColumnItem { entity, position }
    }
}


fn handle_map_spawned(
    mut commands: Commands,
    mut ui_store: ResMut<MapUIStore>,
    query: Query<(Entity, &Dimensions, Added<Map>)>
) {
    for (entity, dimensions, _map) in query.iter() {
        let num_columns = dimensions.width + dimensions.height - 1;

        trace!("handle_map_spawned() - Creating {} UI columns", num_columns);

        for n in 0..=num_columns {
            let left = (n % 2 == 0);

            let column_entity = commands
                .spawn((
                    MapUIColumn { items: vec![], left },
                    Position::new(n / 2, 0),
                    Parent(entity)
                ))
                .with_bundle(MeshComponents::default())
                .current_entity()
                .unwrap();

            ui_store.columns.push(column_entity);
        }
    }
}



/// For each tile spawned, add it into left & right columns
///
/// Handling mouse movement:
/// iterate through columns until finding the one the mouse is in,
/// iterate through tiles from bottom to top checking if the mouse is below the top line
///
fn handle_tile_spawned(
    mut commands: Commands,
    query: Query<(Entity, &Position, Added<Tile>)>,
) {
    for (entity, position, _tile) in query.iter() {
        // trace!("handle_tile_spawned() - insert interaction component");
        commands.insert_one(entity, Interaction::default());

        for i in 0..=1 {
            let column = position.x + position.y + i;
            commands.add_command(AddTileToUIColumnCmd { entity, column });
        }
    }
}



struct AddTileToUIColumnCmd {
    entity: Entity,
    column: i32,
}

impl Command for AddTileToUIColumnCmd {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {
        let ui_store = resources.get::<MapUIStore>().unwrap();

        let position = world.get::<Position>(self.entity).unwrap().clone();
        let column_entity = ui_store.columns[self.column as usize];

        // trace!("handle_tile_spawned() - add to ui column: {}, position: {:?}, column_entity: {:?}", self.column, position, column_entity);

        let mut map_ui_column = world
            .get_mut::<MapUIColumn>(column_entity)
            .unwrap();

        map_ui_column.add(MapUIColumnItem::new(self.entity, position));
    }
}


