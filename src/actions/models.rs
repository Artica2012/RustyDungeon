use bevy::prelude::{Entity, With, World};

use crate::actions::Action;
use crate::board::components::Position;
use crate::board::CurrentBoard;
use crate::pieces::components::Occupier;
use crate::vectors::Vector2Int;

pub struct WalkAction {
    pub entity: Entity,
    pub destination: Vector2Int,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.v == self.destination)
        {
            return false;
        };

        let Some(board) = world.get_resource::<CurrentBoard>() else {
            return false;
        };
        if !board.tiles.contains_key(&self.destination) {
            return false;
        };

        let Some(mut position) = world.get_mut::<Position>(self.entity) else {
            return false;
        };
        position.v = self.destination;
        true
    }
}
