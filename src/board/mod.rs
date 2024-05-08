use std::collections::HashMap;

use bevy::app::App;
use bevy::prelude::{Entity, OnExit, Plugin, Resource};

use crate::board::dungeon::Dungeon;
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;
pub mod dungeon;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .insert_resource::<Dungeon>(Dungeon::new(2))
            .add_systems(OnExit(MainState::LoadAssets), systems::spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}
