use bevy::prelude::*;

use crate::actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent};
use crate::graphics::GraphicsWaitEvent;
use crate::input::PlayerInputReadyEvent;
use crate::player::cards::PlayerActionEvent;
use crate::states::{GameState, MainState, TurnSet};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ((TurnSet::Logic, TurnSet::Animation, TurnSet::Tick).chain())
                .run_if(in_state(GameState::TurnUpdate)),
        )
        .add_systems(OnEnter(MainState::Game), game_start)
        .add_systems(OnExit(MainState::Game), game_end)
        .add_systems(
            Update,
            turn_update_start.run_if(on_event::<PlayerActionEvent>()),
        )
        .add_systems(
            Update,
            turn_update_end.run_if(on_event::<ActionsCompleteEvent>()),
        )
        .add_systems(
            Update,
            turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
        )
        .add_systems(Update, tick.in_set(TurnSet::Tick));
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    println!("Starting Game");
    next_state.set(GameState::PlayerInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::None);
}

fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    info!("Turn Update Start");
    next_state.set(GameState::TurnUpdate);
    ev_tick.send(TickEvent);
}

fn tick(mut ev_wait: EventReader<GraphicsWaitEvent>, mut ev_tick: EventWriter<TickEvent>) {
    if ev_wait.len() == 0 {
        ev_tick.send(TickEvent);
    }
}

fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    info!("Turn Update End");
    next_state.set(GameState::PlayerInput);
}

fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput)
}
