use std::collections::HashMap;

use bevy::prelude::{Component, Entity, Resource};

use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
pub struct Tile;

#[derive(Default, Resource)]
pub struct BoardRes {
    pub tiles: HashMap<Vector2Int, Entity>,
}
