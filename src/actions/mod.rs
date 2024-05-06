use std::any::Any;
use std::collections::VecDeque;

use bevy::ecs::system::ExclusiveSystemParamFunction;
use bevy::prelude::*;

use crate::actions::systems::{plan_melee, plan_walk, populate_actor_queue, process_action_queue};
use crate::states::GameState;

pub(crate) mod models;
mod systems;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_event::<ActionExecutedEvent>()
            .configure_sets(
                Update,
                (
                    ActionSet::Planning.run_if(on_event::<NextActorEvent>()),
                    ActionSet::Planning.before(ActionSet::Late),
                ),
            )
            .add_systems(
                Update,
                process_action_queue
                    .run_if(on_event::<TickEvent>())
                    .in_set(ActionSet::Late),
            )
            .add_systems(OnExit(GameState::PlayerInput), populate_actor_queue)
            .add_systems(Update, (plan_walk, plan_melee).in_set(ActionSet::Planning));
        // .add_systems(Update, plan_walk.run_if(on_event::<NextActorEvent>()));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionSet {
    Planning,
    Late,
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;

#[derive(Event)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);
