use std::collections::HashSet;
use std::vec::Vec;

use bevy::prelude::Resource;

pub use area::Area;

use crate::board::dungeon::tunneler::{LShapeTunneler, RandomTunneler, Tunneler};
use crate::vectors::Vector2Int;

mod area;
pub mod room;
pub mod tunneler;

const AREA_SPACING: i32 = 4;

#[derive(Resource)]
pub struct Dungeon {
    pub areas: Vec<Area>,
    grid: Vec<Vec<usize>>,
}

impl Dungeon {
    pub fn new(row_count: usize) -> Self {
        let grid = (0..row_count).map(|_| Vec::new()).collect::<Vec<_>>();
        Self {
            areas: Vec::new(),
            grid,
        }
    }

    pub fn add_area(&mut self, area: Area) {
        self.areas.push(area);
        let idx = self.areas.len() - 1;
        let row_count = self.grid.len();

        self.grid[idx % row_count].push(idx);
    }

    pub fn generate(&mut self) {
        for (idx, area) in self.areas.iter_mut().enumerate() {
            let tun = match idx % 2 {
                0 => Box::new(LShapeTunneler) as Box<dyn Tunneler>,
                _ => Box::new(RandomTunneler) as Box<dyn Tunneler>,
            };

            area.generate_rooms(tun, Area::get_room_generator());
        }
        self.position_areas();
        self.connect_areas();
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.areas.iter().map(|a| a.to_tiles()).flatten().collect()
    }

    fn position_areas(&mut self) {
        let column_count = self.grid[0].len();

        let column_widths = (0..column_count)
            .map(|i| {
                self.grid
                    .iter()
                    .map(|r| match r.get(i) {
                        None => 0,
                        Some(a) => self.areas[i].get_size().x,
                    })
                    .max()
                    .unwrap()
                    + AREA_SPACING
            })
            .collect::<Vec<_>>();

        let row_heights = self
            .grid
            .iter()
            .map(|r| r.iter().map(|i| self.areas[*i].get_size().y).max().unwrap() + AREA_SPACING)
            .collect::<Vec<_>>();

        // Caluclate the offset amounts per each grid position
        let column_shifts = (0..column_widths.len())
            .map(|i| column_widths[..i].iter().sum())
            .collect::<Vec<i32>>();
        let row_shifts = (0..row_heights.len())
            .map(|i| row_heights[..i].iter().sum())
            .collect::<Vec<i32>>();

        //reposition the areas
        for (y, row) in self.grid.iter().enumerate() {
            for (x, idx) in row.iter().enumerate() {
                let offset = Vector2Int::new(column_shifts[x], row_shifts[y]);
                self.areas[*idx].shift(offset);
            }
        }
    }

    fn connect_areas(&mut self) {
        let mut pairs = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, idx) in row.iter().enumerate() {
                if x != 0 {
                    pairs.push((idx, row[x - 1]));
                };
                if y != 0 {
                    pairs.push((idx, self.grid[y - 1][x]));
                };
            }
        }
        for pair in pairs {
            let path = self.areas[*pair.0].join_area(&self.areas[pair.1], Box::new(RandomTunneler));
            self.areas[*pair.0].paths.push(path);
        }
    }
}
