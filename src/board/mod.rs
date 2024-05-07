use std::collections::HashMap;

use bevy::app::App;
use bevy::prelude::{Entity, OnExit, Plugin, Resource};

use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;
mod dungeon;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(OnExit(MainState::LoadAssets), systems::spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}
