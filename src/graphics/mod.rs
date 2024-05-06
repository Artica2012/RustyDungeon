use bevy::app::App;
use bevy::app::Update;
use bevy::asset::Handle;
use bevy::ecs::event::Event;
use bevy::prelude::{Image, IntoSystemConfigs, OnEnter, Plugin, Resource, Vec3};
use bevy::sprite::TextureAtlasLayout;

use crate::board::components::Position;
use crate::states::{MainState, TurnSet};
use crate::vectors::Vector2Int;

mod assets;
mod components;
mod pieces;
mod tiles;

pub const TILE_SIZE: f32 = 32.;

pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;

const POSITION_TOLERANCE: f32 = 0.1;
const PIECE_SPEED: f32 = 10.;

#[derive(Resource)]
pub struct GraphicsAssets {
    //The texture atlas was reworked in .13, splitting the atlas and texture into two separate components.
    pub sprite_texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_systems(OnEnter(MainState::Game), pieces::spawn_piece_renderer)
            .add_systems(OnEnter(MainState::Game), tiles::spawn_tile_renderer)
            // .add_systems(Update, update_piece_position.in_set(MainState::Game))
            .add_systems(
                Update,
                (
                    pieces::walk_animation,
                    pieces::path_animator_update,
                    pieces::melee_animation,
                )
                    .in_set(TurnSet::Animation),
            );
    }
}

pub fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.v.x as f32,
        TILE_SIZE * position.v.y as f32,
        z,
    )
}

pub fn get_world_vec(v: Vector2Int, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}

#[derive(Event)]
pub struct GraphicsWaitEvent;
