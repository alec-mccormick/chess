use bevy::prelude::*;
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<TileMaterials>()
            .add_startup_system(setup.system())
        ;
    }
}



struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
}

impl FromResources for TileMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        TileMaterials {
            white: materials.add(asset_server.load("white.png").into()),
            black: materials.add(asset_server.load("black.png").into()),
        }
    }
}







#[derive(Debug, Clone, Default)]
pub struct Tile;



fn setup(mut commands: Commands, tile_materials: Res<TileMaterials>) {
    for x in 0..8 {
        for y in 0..8 {
            let position = Position::new(x, y);

            let transform = Transform {
                translation: convert_position_to_translation(&position),
                rotation: Quat::identity(),
                scale: Vec3::splat(0.5)
            };

            let material = if (x + y) % 2 == 0 {
                tile_materials.black.clone()
            } else {
                tile_materials.white.clone()
            };

            commands
                .spawn((
                    Tile,
                    position,
                    Interaction::default(),
                ))
                .with_bundle(SpriteComponents {
                    material,
                    transform,
                    ..Default::default()
                })
            ;
        }
    }
}



fn convert_position_to_translation(position: &Position) -> Vec3 {
    let x = ((position.x as f32) - 4.0) * 100.0 + 50.0;
    let y = ((position.y as f32) - 4.0) * 100.0 + 50.0;
    Vec3::new(x, y, 0.0)
}