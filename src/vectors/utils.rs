use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use crate::vectors::{Vector2Int, ORTHO_DIRECTIONS};

///Currently a modified Dijkstra’s algorithm. In the future, convert it to a A* algorithm
pub fn find_path(
    start: Vector2Int,
    end: Vector2Int,
    tiles: &HashSet<Vector2Int>,
    blockers: &HashSet<Vector2Int>,
) -> Option<VecDeque<Vector2Int>> {
    let mut queue = BinaryHeap::new(); // Create an empty queue
    queue.push(Node {
        //Add the start position as the first node
        v: start,
        cost: 0,
    });
    let mut visited = HashMap::new(); // Create an empty HashMap of visited nodes
    visited.insert(start, 0); // We start out by visiting the starting position
    let mut came_from = HashMap::new(); // This is the linked paths.

    while let Some(Node { v, cost }) = queue.pop() {
        // Work our way through the queue
        if v == end {
            break;
        } //If we have reached the end, break the loop

        for dir in ORTHO_DIRECTIONS {
            //Check each possible direction
            let n = v + dir; // n is the position of the next node (current nodes position + the directional movement)
            let new_cost = cost + 1; // The cost of the new node
            if !tiles.contains(&n) {
                continue;
            } // If the new position isn't on the map, skip adding the node to the lists

            if blockers.contains(&n) && n != end {
                continue;
            } // If the node is non-traversable, skip it
            match visited.get(&n) {
                Some(c) if *c <= new_cost => (), // if the node has not been visited, or the new cost is lower than the established cost, update the visited cost, add the neightbor into the queue and update the came from hashmap

                _ => {
                    visited.insert(n, new_cost);
                    queue.push(Node {
                        v: n,
                        cost: new_cost,
                    });
                    came_from.insert(n, v);
                }
            }
        }
    }
    let mut path = VecDeque::new(); //Create a new VecDeque for the path
    let mut cur = end; // Set current node to the ending node, so we will work backwards through the came_from to generate the path.
    while let Some(v) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *v;
        if cur == start {
            return Some(path);
        }
    }
    None
}

pub fn a_star_pathfinding(
    start: Vector2Int,
    end: Vector2Int,
    tiles: &HashSet<Vector2Int>,
    blockers: &HashSet<Vector2Int>,
) -> Option<VecDeque<Vector2Int>> {
    let mut queue = BinaryHeap::new(); // Create an empty queue
    queue.push(Node {
        //Add the start position as the first node
        v: start,
        cost: 0,
    });
    let mut visited = HashMap::new(); // Create an empty HashMap of visited nodes
    visited.insert(start, 0); // We start out by visiting the starting position
    let mut came_from = HashMap::new(); // This is the linked paths.

    while let Some(Node { v, cost }) = queue.pop() {
        // Work our way through the queue
        if v == end {
            break;
        } //If we have reached the end, break the loop

        for dir in ORTHO_DIRECTIONS {
            //Check each possible direction
            let n = v + dir; // n is the position of the next node (current nodes position + the directional movement)
            let new_cost = cost + 1; // The cost of the new node
            if !tiles.contains(&n) {
                continue;
            } // If the new position isn't on the map, skip adding the node to the lists

            if blockers.contains(&n) && n != end {
                continue;
            } // If the node is non-traversable, skip it
            let estimated_total_cost = new_cost + n.manhattan(end) as u32;

            match visited.get(&n) {
                Some(c) if *c <= new_cost => (), // if the node has not been visited, or the new cost is lower than the established cost, update the visited cost, add the neightbor into the queue and update the came from hashmap

                _ => {
                    visited.insert(n, new_cost);
                    queue.push(Node {
                        v: n,
                        cost: estimated_total_cost,
                    });
                    came_from.insert(n, v);
                }
            }
        }
    }
    let mut path = VecDeque::new(); //Create a new VecDeque for the path
    let mut cur = end; // Set current node to the ending node, so we will work backwards through the came_from to generate the path.
    while let Some(v) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *v;
        if cur == start {
            return Some(path);
        }
    }
    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pub v: Vector2Int,
    pub cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
