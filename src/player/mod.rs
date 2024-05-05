use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::{Actor, ActorQueue};
use crate::board::components::Position;
use crate::pieces::components::{Health, Piece};
use crate::player::cards::{DeckEvent, DeckEventKind, PlayerActionEvent};
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod cards;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Deck::default())
            .add_event::<PlayerActionEvent>()
            .add_event::<DeckEvent>()
            .add_systems(OnExit(MainState::LoadAssets), spawn_player)
            .add_systems(Update, dispatch_card.run_if(on_event::<DeckEvent>()))
            .add_systems(Update, select_card.run_if(on_event::<DeckEvent>()));
    }
}

#[derive(Default, Resource)]
pub struct Deck {
    pub cards: Vec<Entity>,
    pub current_card: Option<Entity>,
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    let walk_card = commands
        .spawn(cards::CardHolder(Box::new(cards::WalkCard)))
        .id();
    let melee_card = commands
        .spawn(cards::CardHolder(Box::new(cards::MeleeCard(3))))
        .id();

    commands.insert_resource(Deck {
        cards: vec![walk_card, melee_card],
        ..default()
    });

    commands.spawn((
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position {
            v: Vector2Int::new(0, 0),
        },
        Actor::default(),
        Health { value: 20 },
    ));
}

pub fn select_card(mut ev_deck: EventReader<DeckEvent>, mut deck: ResMut<Deck>) {
    for ev in ev_deck.read() {
        if let DeckEvent(DeckEventKind::SelectCard(entity)) = ev {
            deck.current_card = Some(*entity);
        }
    }
}

pub fn dispatch_card(
    mut ev_deck: EventReader<DeckEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,
    deck: Res<Deck>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
    card_query: Query<&cards::CardHolder>,
    mut queue: ResMut<ActorQueue>,
) {
    for ev in ev_deck.read() {
        if let DeckEvent(DeckEventKind::UseCard(v)) = ev {
            let Ok((entity, mut actor)) = player_query.get_single_mut() else {
                return;
            };
            let Some(card_entity) = deck.current_card else {
                return;
            };
            let Ok(card) = card_query.get(card_entity) else {
                return;
            };
            let Some(action) = card.0.get_action(entity, *v) else {
                continue;
            };

            //action score doesn't matter for the player
            actor.0 = vec![(action, 0)];

            //Player moves first, so start with a single element queue
            queue.0 = VecDeque::from([entity]);
            ev_action.send(PlayerActionEvent);
        }
    }
}
