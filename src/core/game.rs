use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::unit::*;
use super::map::*;
use crate::{prelude::*, units::*};


/// ==========================================================================
/// Game Descriptor
/// ==========================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDescriptor {
    pub map: MapDescriptor,
    pub units: Vec<(Team, Unit, Position, Id)>,
}

impl EntitySpawner for GameDescriptor {
    fn spawn(self, commands: &mut Commands) -> &mut Commands {
        let GameDescriptor { map, units } = self;

        map.spawn(commands).with_children(|commands| {
            for descriptor in units.into_iter() {
                let components = convert_unit_descriptor_to_components(descriptor);
                commands.spawn(components);
            }
        })
    }
}

impl Default for GameDescriptor {
    fn default() -> Self {
        let map = MapDescriptor::default();
        let mut units = Vec::new();

        for x in 0..=7 {
            units.push((Team::White, Unit::Pawn, (x, 1).into(), Id::new()));
            units.push((Team::Black, Unit::Pawn, (x, 6).into(), Id::new()));
        }

        let team = Team::White;
        let home_row = 0;
        units.push((team, Unit::Rook, (0, home_row).into(), Id::new()));
        units.push((team, Unit::Knight, (1, home_row).into(), Id::new()));
        units.push((team, Unit::Bishop, (2, home_row).into(), Id::new()));
        units.push((team, Unit::Queen, (3, home_row).into(), Id::new()));
        units.push((team, Unit::King, (4, home_row).into(), Id::new()));
        units.push((team, Unit::Bishop, (5, home_row).into(), Id::new()));
        units.push((team, Unit::Knight, (6, home_row).into(), Id::new()));
        units.push((team, Unit::Rook, (7, home_row).into(), Id::new()));

        let team = Team::Black;
        let home_row = 7;
        units.push((team, Unit::Rook, (0, home_row).into(), Id::new()));
        units.push((team, Unit::Knight, (1, home_row).into(), Id::new()));
        units.push((team, Unit::Bishop, (2, home_row).into(), Id::new()));
        units.push((team, Unit::Queen, (3, home_row).into(), Id::new()));
        units.push((team, Unit::King, (4, home_row).into(), Id::new()));
        units.push((team, Unit::Bishop, (5, home_row).into(), Id::new()));
        units.push((team, Unit::Knight, (6, home_row).into(), Id::new()));
        units.push((team, Unit::Rook, (7, home_row).into(), Id::new()));

        GameDescriptor { map, units }
    }
}

// ==========================================================================
// -- Helper Functions
// ==========================================================================
fn convert_unit_descriptor_to_components((team, unit, position, id): (Team, Unit, Position, Id)) -> UnitComponents {

    match unit {
        Unit::Pawn => UnitComponents { team, position, id, ..pawn() },
        Unit::Bishop => UnitComponents { team, position, id, ..bishop() },
        Unit::Knight => UnitComponents { team, position, id, ..knight() },
        Unit::Rook => UnitComponents { team, position, id, ..rook() },
        Unit::King => UnitComponents { team, position, id, ..king() },
        Unit::Queen => UnitComponents { team, position, id, ..queen() }
    }
}