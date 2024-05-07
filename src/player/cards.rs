use bevy::prelude::*;

use crate::actions::models::{MeleeHitAction, WalkAction};
use crate::actions::Action;
use crate::vectors::Vector2Int;

pub trait Card: Send + Sync {
    fn get_action(&self, owner: Entity, target: Option<Vector2Int>) -> Option<Box<dyn Action>>;
    fn get_label(&self) -> String;
}

#[derive(Component)]
pub struct CardHolder(pub Box<dyn Card>);

pub struct WalkCard;

impl Card for WalkCard {
    fn get_action(&self, owner: Entity, target: Option<Vector2Int>) -> Option<Box<dyn Action>> {
        Some(Box::new(WalkAction {
            entity: owner,
            destination: target?,
        }))
    }

    fn get_label(&self) -> String {
        "Walk".into()
    }
}

pub struct MeleeCard(pub u32);

impl Card for MeleeCard {
    fn get_action(&self, owner: Entity, target: Option<Vector2Int>) -> Option<Box<dyn Action>> {
        Some(Box::new(MeleeHitAction {
            attacker: owner,
            target: target?,
            damage: self.0,
        }))
    }

    fn get_label(&self) -> String {
        format!("Melee\n{} dmg", self.0)
    }
}

pub enum DeckEventKind {
    SelectCard(Entity),
    UseCard(Option<Vector2Int>),
}

#[derive(Event)]
pub struct DeckEvent(pub DeckEventKind);

#[derive(Event)]
pub struct PlayerActionEvent;
