use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::{models::WalkAction, Actor, ActorQueue};
use crate::board::components::Position;
use crate::player::cards::{DeckEvent, DeckEventKind};
use crate::player::{Deck, Player};
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>().add_systems(
            Update,
            player_input.run_if(in_state(GameState::PlayerInput)),
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

//Depreciated
fn player_position(
    mut keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        println!("Bad Input Query");
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        // println!("Key Pressed {:?}", dir);
        let action = WalkAction {
            entity: entity,
            destination: position.v + dir,
        };
        actor.0 = vec![(Box::new(action), 0)];
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&Position, With<Player>>,
    deck: Res<Deck>,
    mut ev_deck: EventWriter<DeckEvent>,
) {
    let Ok(position) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        ev_deck.send(DeckEvent(DeckEventKind::UseCard(Some(position.v + dir))));
    }

    if keys.just_pressed(KeyCode::Digit1) {
        if let Some(entity) = deck.cards.get(0) {
            ev_deck.send(DeckEvent(DeckEventKind::SelectCard(*entity)));
        }
    }

    if keys.just_pressed(KeyCode::Digit2) {
        if let Some(entity) = deck.cards.get(1) {
            ev_deck.send(DeckEvent(DeckEventKind::SelectCard(*entity)));
        }
    }
}
