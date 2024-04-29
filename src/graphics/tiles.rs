use bevy::prelude::*;

use crate::board::components::{Position, Tile};

use super::{GraphicsAssets, TILE_SIZE};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let v = Vec3::new(
            TILE_SIZE * position.v.x as f32,
            TILE_SIZE * position.v.y as f32,
            0.,
        );

        commands.entity(entity).insert(SpriteSheetBundle {
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
