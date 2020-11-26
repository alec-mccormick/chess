use bevy::prelude::*;
use log::{debug};
use crate::prelude::*;

use std::cmp::max;
use crate::core::map::{Map};

use super::super::utils;
use bevy::ecs::Command;


/// Element to contain map and control map scale.
#[derive(Clone, Copy)]
pub struct MapContainer;

/// Command to make a draw a map & position it in the screen.
#[derive(Clone, Copy)]
pub struct RenderMapCmd {
    pub entity: Entity,
}

impl Command for RenderMapCmd {
    fn write(self: Box<Self>, world: &mut World, _: &mut Resources) {
        let map_container_entity = self.spawn_container(world);
        self.insert_map_mesh(world, map_container_entity);
    }
}

impl RenderMapCmd {
    fn spawn_container(&self, world: &mut World) -> Entity {
        let map_container_entity = world.spawn(MeshComponents {
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        });

        world.insert_one(map_container_entity, MapContainer).unwrap();
        map_container_entity
    }

    fn insert_map_mesh(&self, world: &mut World, parent: Entity) {
        let dimensions = world.get::<Dimensions>(self.entity).unwrap();

        world.insert(self.entity, MeshComponents {
            transform: Transform {
                translation: utils::convert_dimensions_to_map_offset(dimensions),
                ..Default::default()
            },
            ..Default::default()
        }).unwrap();

        world.insert_one(self.entity, Parent(parent)).unwrap();
    }
}