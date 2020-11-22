use bevy::{prelude::*};
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
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,

        query: Query<Without<Sprite, (Entity, &Position, &SpriteConfig)>>,
    ) {
        for (entity, position, sprite_config) in query.iter() {
            println!("handle entity spawned: {:?}", entity);

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

    fn handle_position_changed(
        mut query: Query<(Mutated<Position>, &mut Transform)>,
    ) {
        for (position, mut transform) in query.iter_mut() {
            println!("Graphics.handle_position_changed()");

            transform.translation = convert_position_to_translation(&*position);
        }
    }
}

fn convert_position_to_translation(position: &Position) -> Vec3 {
    let x = ((position.x as f32) - 4.0) * 100.0 + 50.0;
    let y = ((position.y as f32) - 4.0) * 100.0 + 50.0;
    Vec3::new(x, y, 0.0)
}