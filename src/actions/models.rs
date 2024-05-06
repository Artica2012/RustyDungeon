use std::any::Any;

use bevy::prelude::{info, Entity, With, World};

use crate::actions::Action;
use crate::board::components::Position;
use crate::board::CurrentBoard;
use crate::pieces::components::{Health, Occupier};
use crate::vectors::Vector2Int;

pub struct WalkAction {
    pub entity: Entity,
    pub destination: Vector2Int,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let board = world.get_resource::<CurrentBoard>().ok_or(())?;
        if !board.tiles.contains_key(&self.destination) {
            return Err(());
        };
        info!("Walking");
        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.v == self.destination)
        {
            return Err(());
        };

        let mut position = world.get_mut::<Position>(self.entity).ok_or(())?;

        position.v = self.destination;
        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MeleeHitAction {
    pub attacker: Entity,
    pub target: Vector2Int,
    pub damage: u32,
}

impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        info!("Attacking");
        let attk_pos = world.get::<Position>(self.attacker);
        println!("{:?}", attk_pos.unwrap().v);
        println!("{:?}", self.target);
        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;
        println!("Didn't error");
        println!("Distance: {}", attacker_position.v.manhattan(self.target));
        if attacker_position.v.manhattan(self.target) > 1 {
            return Err(());
        };

        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Health>>()
            .iter(world)
            .filter(|(_, p)| p.v == self.target)
            .collect::<Vec<_>>();
        println!("Number of viable targets{}", &target_entities.len());
        if target_entities.len() == 0 {
            return Err(());
        };
        let result = target_entities
            .iter()
            .map(|e| {
                Box::new(DamageAction {
                    entity: e.0,
                    damage: self.damage,
                }) as Box<dyn Action>
            })
            .collect::<Vec<_>>();
        info!("Hit!");
        Ok(result)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct DamageAction {
    entity: Entity,
    damage: u32,
}

impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(mut health) = world.get_mut::<Health>(self.entity) else {
            return Err(());
        };
        health.value = health.value.saturating_sub(self.damage);

        println!("{:?}", &health.value);

        if health.value == 0 {
            world.despawn(self.entity);
        }

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
