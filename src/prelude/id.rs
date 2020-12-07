use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Id(uuid)
    }

    pub fn to_uuid(&self) -> &Uuid {
        &self.0
    }
}