use std::collections::VecDeque;

use bevy::ecs::system::ExclusiveSystemParamFunction;
use bevy::prelude::*;

use crate::actions::systems::process_action_queue;

pub(crate) mod models;
mod systems;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()));
    }
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> bool;
}

#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;
