use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, OnExit};

use crate::actions::Actor;
use crate::board::components::Position;
use crate::pieces::components::{Health, Melee, Occupier, Piece, Walk};
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(MainState::LoadAssets), spawn_npcs);
    }
}

pub fn spawn_npcs(mut commands: Commands) {
    println!("Spawning NPCs");
    commands.spawn((
        Actor::default(),
        Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int::new(3, 5),
        },
        Walk,
        Occupier,
        Melee { damage: 5 },
        Health { value: 5 },
    ));
    commands.spawn((
        Actor::default(),
        Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int::new(5, 5),
        },
        Walk,
        Occupier,
        Melee { damage: 5 },
        Health { value: 5 },
    ));
}
