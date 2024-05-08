use std::collections::HashSet;

use rand::prelude::*;

use crate::vectors::Vector2Int;

pub struct Room {
    pub a: Vector2Int,
    pub b: Vector2Int,
}

pub trait RoomGenerator {
    fn generate(&self) -> GeneratorResult;
}

pub struct GeneratorResult {
    pub rooms: Vec<Room>,
    pub connections: Vec<(usize, usize)>,
}

impl Room {
    pub fn new(a: Vector2Int, b: Vector2Int) -> Self {
        // a is always left bottom, and b is already top-right
        Self {
            a: Vector2Int::new(a.x.min(b.x), a.y.min(b.y)),
            b: Vector2Int::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }

    pub fn corners(&self) -> [Vector2Int; 4] {
        [
            Vector2Int::new(self.a.x, self.a.y),
            Vector2Int::new(self.b.x, self.a.y),
            Vector2Int::new(self.b.x, self.b.y),
            Vector2Int::new(self.a.x, self.b.y),
        ]
    }

    pub fn random_point(&self) -> Vector2Int {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.a.x..=self.b.x);
        let y = rng.gen_range((self.a.y..=self.b.y));
        Vector2Int::new(x, y)
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        (self.a.y..=self.b.y)
            .map(|y| (self.a.x..=self.b.x).map(move |x| Vector2Int::new(x, y)))
            .flatten()
            .collect()
    }

    pub fn center(&self) -> Vector2Int {
        Vector2Int::new((self.b.x + self.a.x) / 2, (self.b.y + self.a.y) / 2)
    }

    pub fn intersects(&self, other: &Room, border: Option<u32>) -> bool {
        let b = match border {
            Some(a) => a as i32,
            None => 0,
        };
        !(other.a.x > self.b.x + b
            || other.b.x < self.a.x - b
            || other.a.y > self.b.y + b
            || other.b.y < self.a.y - b)
    }
}

pub struct BubbleGenerator {
    pub room_count: (u32, u32),
    pub room_size: (u32, u32),
    pub room_padding: Option<u32>,
    pub extra_connection_chance: f64,
}

impl BubbleGenerator {
    fn random_dim(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        (
            rng.gen_range(self.room_size.0..self.room_size.1) as i32,
            rng.gen_range(self.room_size.0..=self.room_size.1) as i32,
        )
    }
}

impl RoomGenerator for BubbleGenerator {
    fn generate(&self) -> GeneratorResult {
        let mut rng = thread_rng();
        let mut connections = Vec::new();

        let (w, h) = self.random_dim();
        let mut rooms = vec![Room::new(Vector2Int::new(0, 0), Vector2Int::new(w, h))];
        let max_dist = self.room_size.1 as i32;

        let count = rng.gen_range(self.room_count.0..self.room_count.1);
        for _ in 0..count {
            loop {
                let prev_idx = rng.gen_range(0..rooms.len());
                let center = rooms[prev_idx].center();
                let a = Vector2Int::new(
                    rng.gen_range(center.x - max_dist..=center.x + max_dist),
                    rng.gen_range(center.y - max_dist..=center.y + max_dist),
                );

                let (w, h) = self.random_dim();
                let b = Vector2Int::new(
                    a.x + *[-w, w].choose(&mut rng).unwrap(),
                    a.y + *[-h, h].choose(&mut rng).unwrap(),
                );

                let r = Room::new(a, b);

                if rooms
                    .iter()
                    .any(|other| r.intersects(other, self.room_padding))
                {
                    continue;
                }
                connections.push((prev_idx, rooms.len()));

                if rng.gen_bool(self.extra_connection_chance) {
                    connections.push((rng.gen_range(0..rooms.len()), rooms.len()));
                }
                rooms.push(Room::new(a, b));
                break;
            }
        }
        GeneratorResult { rooms, connections }
    }
}

// impl Sync for BubbleGenerator{}
//
// impl Send for BubbleGenerator {}
