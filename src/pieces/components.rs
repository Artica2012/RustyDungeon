use bevy::prelude::*;

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

#[derive(Component)]
pub struct Walk;

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component)]
pub struct Melee {
    // Melee attack behavior
    pub damage: u32,
}

#[derive(Component)]
pub struct Occupier;
