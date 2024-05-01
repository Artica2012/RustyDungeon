use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::{models::WalkAction, Actor, ActorQueue};
use crate::board::components::Position;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>().add_systems(
            Update,
            player_position.run_if(in_state(GameState::PlayerInput)),
        );
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::KeyW, Vector2Int::UP),
    (KeyCode::KeyS, Vector2Int::DOWN),
    (KeyCode::KeyA, Vector2Int::LEFT),
    (KeyCode::KeyD, Vector2Int::RIGHT),
];

#[derive(Event)]
pub struct PlayerInputReadyEvent;

fn player_position(
    mut keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    // println!("{:?}", keys);
    if let x = keys.get_just_pressed() {
        for i in x.into_iter() {
            println!("{:?}", i);
        }
    }

    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        println!("Bad Input Query");
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if keys.just_pressed(key) {
            println!("{:?} Presed", key);
        }
        if !keys.just_pressed(key) {
            continue;
        }
        println!("Key Pressed {:?}", dir);
        let action = WalkAction {
            entity: entity,
            destination: position.v + dir,
        };
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}
