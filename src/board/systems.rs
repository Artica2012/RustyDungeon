use std::collections::HashMap;

use bevy::prelude::{Commands, ResMut};

use crate::board::components::{Position, Tile};
use crate::board::dungeon::room;
use crate::board::dungeon::room::{BuggleGenerator, RoomGenerator};
use crate::board::dungeon::tunneler::{LShapeTunneler, RandomTunneler, Tunneler};
use crate::board::dungeon::{Area, Dungeon};
use crate::board::CurrentBoard;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    let mut dungeon = Dungeon::new(2);
    for idx in 0..4 {
        let tun = match idx % 2 {
            0 => Box::new(LShapeTunneler) as Box<dyn Tunneler>,
            _ => Box::new(RandomTunneler) as Box<dyn Tunneler>,
        };

        let gen = Box::new(BuggleGenerator {
            room_count: (3, 5),
            room_size: (4, 8),
            room_padding: Some(2),
            extra_connection_chance: 0.25,
        }) as Box<dyn room::RoomGenerator>;

        dungeon.add_area(Area::new(tun, gen))
    }
    dungeon.generate();

    current.tiles = HashMap::new();

    for v in dungeon.to_tiles() {
        let tile = commands.spawn((Position { v }, Tile)).id();
        current.tiles.insert(v, tile);
    }
}
