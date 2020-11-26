use bevy::prelude::*;
use crate::prelude::*;

use super::tile::*;

#[derive(Debug, Bundle)]
pub struct MapComponents {
    pub map: Map,
    pub dimensions: Dimensions,
}

impl Default for MapComponents {
    fn default() -> Self {
        MapComponents {
            map: Map,
            dimensions: Dimensions::new(8, 8),
        }
    }
}

#[derive(Debug)]
pub struct Map;

