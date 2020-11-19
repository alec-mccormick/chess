use bevy::{app::Events, prelude::*};
use std::marker::PhantomData;
use crate::prelude::*;
use super::types::{UnitStore, UnitHealth};

/// Actual Events:
///
#[derive(Debug, Copy, Clone)]
pub struct PositionChanged(pub Position);

#[derive(Debug, Copy, Clone)]
pub struct HealthChanged(pub UnitHealth);

/// Actual Events:
///
#[derive(Debug, Copy, Clone)]
pub enum UnitCmd {
    SetPosition(Position),
    SetHealth(UnitHealth),
    ExecuteAction(u16, Position)
}



