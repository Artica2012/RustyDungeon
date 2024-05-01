use bevy::prelude::*;

use crate::actions::Actor;
use crate::board::components::Position;
use crate::pieces::components::Piece;
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(MainState::LoadAssets), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position {
            v: Vector2Int::new(0, 0),
        },
        Actor { 0: None },
    ));
}
