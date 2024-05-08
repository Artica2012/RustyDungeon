use std::collections::HashSet;

use crate::board::dungeon::tunneler::Tunneler;
use crate::vectors::Vector2Int;

use super::room::{BubbleGenerator, Room, RoomGenerator};

pub struct Area {
    pub rooms: Vec<Room>,
    pub paths: Vec<Vec<Vector2Int>>,
    // pub tunnler: Box<dyn Tunneler>,
    // pub room_generator: Box<dyn RoomGenerator>,
}

impl Area {
    // pub fn new(tunnler: Box<dyn Tunneler>, room_generator: Box<dyn RoomGenerator>) -> Self {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            paths: Vec::new(),
            // tunnler,
            // room_generator,
        }
    }

    pub fn get_bounds(&self) -> (Vector2Int, Vector2Int) {
        let min_x = self.rooms.iter().map(|r| r.a.x).min().unwrap();
        let max_x = self.rooms.iter().map(|r| r.b.x).max().unwrap();
        let min_y = self.rooms.iter().map(|r| r.a.y).min().unwrap();
        let max_y = self.rooms.iter().map(|r| r.b.y).max().unwrap();
        (Vector2Int::new(min_x, min_y), Vector2Int::new(max_x, max_y))
    }

    pub fn get_size(&self) -> Vector2Int {
        let bounds = self.get_bounds();
        Vector2Int::new(bounds.1.x - bounds.0.x + 1, bounds.1.y - bounds.0.y + 1)
    }

    pub fn shift(&mut self, offset: Vector2Int) {
        let bounds = self.get_bounds();
        let d = offset - bounds.0;

        for room in self.rooms.iter_mut() {
            room.a += d;
            room.b += d;
        }
        for path in self.paths.iter_mut() {
            for v in path.iter_mut() {
                *v += d;
            }
        }
    }

    pub fn generate_rooms(
        &mut self,
        tunnler: Box<dyn Tunneler>,
        room_generator: Box<dyn RoomGenerator>,
    ) {
        let result = room_generator.generate();
        self.rooms = result.rooms;
        self.paths = result
            .connections
            .iter()
            .map(|a| self.join_rooms(&self.rooms[a.0], &self.rooms[a.1], &tunnler))
            .collect();
    }

    //Made this a function to make it easier to tweak the setting. Probably add some sort a randomization to it in the future
    pub fn get_room_generator() -> Box<dyn RoomGenerator> {
        Box::new(BubbleGenerator {
            room_count: (3, 5),
            room_size: (4, 8),
            room_padding: Some(2),
            extra_connection_chance: 0.25,
        }) as Box<dyn RoomGenerator>
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.rooms
            .iter()
            .map(|r| r.to_tiles())
            .flatten()
            .chain(self.paths.iter().flatten().map(|v| *v))
            .collect()
    }

    pub fn join_rooms(&self, a: &Room, b: &Room, tunnler: &Box<dyn Tunneler>) -> Vec<Vector2Int> {
        tunnler.connect(a.random_point(), b.random_point())
    }

    fn find_closest_room_pair<'a>(&'a self, other: &'a Area) -> (&'a Room, &'a Room) {
        let mut pairs = Vec::new();
        for ra in self.rooms.iter() {
            for rb in other.rooms.iter() {
                let d = ra
                    .corners()
                    .iter()
                    .map(|ca| {
                        rb.corners()
                            .iter()
                            .map(|cb| ca.manhattan(*cb))
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .min()
                    .unwrap();
                pairs.push((d, ra, rb));
            }
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        (pairs[0].1, pairs[0].2)
    }

    pub fn join_area(&self, other: &Area, tunneler: Box<dyn Tunneler>) -> Vec<Vector2Int> {
        let rooms = self.find_closest_room_pair(other);
        self.join_rooms(rooms.0, rooms.1, &tunneler)
    }
}
