use bevy::app::{App, PostStartup, Startup};
use bevy::asset::Handle;
use bevy::prelude::{Image, Plugin, Resource};
use bevy::sprite::TextureAtlasLayout;

mod assets;
mod components;
mod tiles;

pub const TILE_SIZE: f32 = 32.;

#[derive(Resource)]
struct GraphicsAssets {
    //The texture atlas was reworked in .13, splitting the atlas and texture into two separate components.
    pub sprite_texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, assets::load_assets)
            .add_systems(PostStartup, tiles::spawn_tile_renderer);
    }
}
