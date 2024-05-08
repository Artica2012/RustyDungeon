use std::collections::HashMap;

use bevy::prelude::*;

use crate::states::GameState;
use crate::ui::deck::{card_click, draw_deck};

mod deck;
mod helpers;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_systems(Update, helpers::button_click_animation)
            .add_systems(OnEnter(GameState::PlayerInput), player_input_start)
            .add_systems(PostUpdate, draw_deck.run_if(on_event::<ReloadUiEvent>()))
            // .add_systems(PostUpdate, draw_deck)
            .add_systems(
                PreUpdate,
                card_click.run_if(in_state(GameState::PlayerInput)),
            );
    }
}

#[derive(Event)]
pub struct ReloadUiEvent;

fn player_input_start(mut ev_ui: EventWriter<ReloadUiEvent>) {
    ev_ui.send(ReloadUiEvent);
}

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
}
