use bevy::prelude::*;

use crate::board::components::Position;
use crate::graphics::{GraphicsAssets, PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE};
use crate::pieces::components::Piece;

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<GraphicsAssets>,
) {
    println!("Spawning Player");
    println!("{:?}", query);
    for (entity, position, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 1,
            _ => 63,
        };

        let v = super::get_world_position(&position, PIECE_Z);

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(32.)),
                ..default()
            },
            atlas: TextureAtlas {
                layout: assets.atlas.clone(),
                index: sprite_idx,
            },
            texture: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..default()
        });
    }
}

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    let mut animating = false;

    for (position, mut transform) in query.iter_mut() {
        let target = super::get_world_position(&position, PIECE_Z);
        let d = (target - transform.translation).length();

        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_seconds());
            animating = true;
        } else {
            transform.translation = target;
        }
        if animating {
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}
