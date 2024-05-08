use bevy::prelude::*;

use crate::board::components::{Position, Tile};

use super::{GraphicsAssets, TILE_Z};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    println! {"Starting to spawn tiles"};
    // println!("{:?}", query);
    for (entity, position) in query.iter() {
        let v = super::get_world_position(&position, TILE_Z);
        // println!("{:?}", v);

        // println!("Spawning Tile Entity");
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                color: Color::GOLD,

                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::splat(32.)),
                rect: None,
                anchor: Default::default(),
            },
            atlas: TextureAtlas {
                layout: assets.atlas.clone(),
                index: 177,
            },
            texture: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..default()
        });
    }
}
