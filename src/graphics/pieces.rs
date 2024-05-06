use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::models::{MeleeHitAction, WalkAction};
use crate::actions::ActionExecutedEvent;
use crate::board::components::Position;
use crate::graphics::components::PathAnimator;
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

//Depreciated in favor of path_animator_update
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

pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.0.len() == 0 {
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }

        ev_wait.send(super::GraphicsWaitEvent);
        let target = *animator.0.get(0).unwrap();
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_seconds());
        } else {
            transform.translation = target;
            animator.0.pop_front();
        }
    }
}

pub fn walk_animation(
    mut commands: Commands,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let target = super::get_world_vec(action.destination, PIECE_Z);
            commands
                .entity(action.entity)
                .insert(PathAnimator(VecDeque::from([target])));
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

pub fn melee_animation(
    mut commands: Commands,
    query: Query<&Position>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeHitAction>() {
            info!("Melee Attack Action");
            let Ok(base_position) = query.get(action.attacker) else {
                continue;
            };
            let base = super::get_world_position(base_position, PIECE_Z);
            let target = 0.5 * (base + super::get_world_vec(action.target, PIECE_Z));
            commands
                .entity(action.attacker)
                .insert(PathAnimator(VecDeque::from([target, base])));
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}
