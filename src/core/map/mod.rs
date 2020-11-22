use bevy::prelude::*;
use crate::prelude::*;




#[derive(Debug, Bundle)]
pub struct TileComponents {
    pub tile: Tile,
    pub position: Position,
}


#[derive(Debug, Clone)]
pub enum Tile {
    Black,
    White,
}




#[derive(Debug, Clone)]
pub struct MapPlugin;


impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
        ;
    }
}


fn setup(mut commands: Commands) {
    for x in 0..8 {
        for y in 0..8 {

            let position = Position::new(x, y);
            let tile = if (x + y) % 2 == 0 { Tile::Black } else { Tile::White };

            commands.spawn(TileComponents { tile, position });
        }
    }
}




