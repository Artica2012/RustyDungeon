use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, OnEnter, Res};

use crate::actions::Actor;
use crate::board::components::Position;
use crate::board::dungeon::Dungeon;
use crate::pieces::components::{Health, Melee, Occupier, Piece, Walk};
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::GenerateMap), spawn_npcs);
    }
}

pub fn spawn_npcs(mut commands: Commands, dungeon: Res<Dungeon>) {
    println!("Spawning NPCs");

    for (a_idx, a) in dungeon.areas.iter().enumerate() {
        for (r_idx, r) in a.rooms.iter().enumerate() {
            println!("Room {a_idx}, {r_idx}");
            if (a_idx != 0) || (r_idx != 0) {
                println!("Spawning");
                // if its not the first room in the fist area
                spawn_generic_npc(&mut commands, r.random_point());
            }
        }
    }
}

fn spawn_generic_npc(mut commands: &mut Commands, location: Vector2Int) {
    println!("{:?}", location);
    commands.spawn((
        Actor::default(),
        Piece {
            kind: "NPC".to_string(),
        },
        Position { v: location },
        Walk,
        Occupier,
        Melee { damage: 5 },
        Health { value: 5 },
    ));
}
