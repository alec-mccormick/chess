use bevy::{prelude::*};
use derive_more::{From, Into, Deref};
use crate::prelude::*;

use super::unit::{self, UnitType};




pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(GraphicsPlugin::setup.system())
            .add_system(GraphicsPlugin::handle_entity_with_sprite_spawned.system())
            .add_system(GraphicsPlugin::handle_position_changed.system())
            // .add_system(animate_sprite_system.system())
        ;
    }
}


impl GraphicsPlugin {

    fn setup(mut commands: Commands) {
        commands
            .spawn(Camera2dComponents::default())
        ;
    }

    fn handle_entity_with_sprite_spawned(
        mut commands: Commands,
        mut reader: Local<EventReader<entity::events::EntitySpawned>>,
        events: Res<Events<entity::events::EntitySpawned>>,

        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,

        query: Query<(&Position, &SpriteConfig)>,
    ) {
        for event in reader.iter(&events) {
            let (object_id, entity) = (event.0, event.1);

            println!("handle entity spawned: {:?}", object_id);

            if let Ok(result) = query.get(entity) {
                let (position, sprite_config) = result;

                let texture_handle = asset_server.load(sprite_config.src.as_str());

                let transform = Transform {
                    translation: convert_position_to_translation(position),
                    rotation: Quat::identity(),
                    scale: Vec3::splat(2.0)
                };

                commands.insert(entity, SpriteComponents {
                    material: materials.add(texture_handle.into()),
                    transform,
                    ..Default::default()
                });
            }
        }
    }

    fn handle_position_changed(
        mut reader: Local<EventReader<(Entity, unit::components::events::PositionChanged)>>,
        events: Res<Events<(Entity, unit::components::events::PositionChanged)>>,
        mut query: Query<(&mut Transform)>,
    ) {
        for (entity, event) in reader.iter(&events) {
            println!("Graphics.handle_position_changed()");

            let mut transform = query.get_mut(*entity).unwrap();
            transform.translation = convert_position_to_translation(&event.0);
        }
    }
}

fn convert_position_to_translation(position: &Position) -> Vec3 {
    let x = ((position.x as f32) - 4.0) * 100.0 + 50.0;
    let y = ((position.y as f32) - 4.0) * 100.0 + 50.0;
    Vec3::new(x, y, 0.0)
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}