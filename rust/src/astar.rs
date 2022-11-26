use std::collections::{HashSet, HashMap};

use priority_queue::PriorityQueue;

use crate::{position::{Position, self}, rotation::Rotation};


pub struct Astar {
    begin: Position,
    end: Position,
    evade: HashSet<Position>,
}

impl Astar {
    pub fn new(begin: Position, end: Position, evade: HashSet<Position>) -> Self {
        Astar { begin, end, evade}
    }

    pub fn construct_path(&self, node: Position, parents: &HashMap<Position, Position>) -> Vec<Position> {
        let mut path = vec![node];
        let mut current_node = node;
        while let Some(parent) = parents.get(&current_node) {
            current_node = parent.clone();
            path.push(current_node);
        }
        path
    }

    pub fn astar_belt(&self) -> Option<Vec<Position>> {
        let mut open_list = PriorityQueue::new();
        let mut closed_list = HashSet::new();
        let mut parents = HashMap::new();
        let mut gs = HashMap::new();
        gs.insert(self.begin, 0);

        for successor in self.belt_successors(self.begin) {
            gs.insert(successor, 0);
            open_list.push(successor, -self.f(successor, &gs));
        }

        while let Some((node, f)) = open_list.pop() {
            let successor_cost = gs.get(&node).unwrap() + 1;
            if successor_cost > 500 {
                return None;
            }

            for successor in self.belt_successors(node).drain(0..) {
                if successor == self.end {
                    parents.insert(successor, node);
                    return Some(self.construct_path(successor, &parents));
                }

                if successor.y < 0 {
                    continue;
                }

                let mut successor_north = successor; // Check if we can even add nodes here.
                successor_north.rotation = Rotation::North;
                if self.evade.contains(&successor_north) {
                    continue;
                }

                // Check if there already is a fast path from successor to here
                if let Some(g) = gs.get(&successor) {
                    if *g < successor_cost {
                        continue;
                    }
                }

                gs.insert(successor, successor_cost);
                // Move successor from closed to open if this is a faster path
                closed_list.remove(&successor);
                parents.insert(successor, node);
                open_list.push(successor, -self.f(successor, &gs));
            }
            closed_list.insert(node);
        }

        None
    }

    pub fn f(&self, node: Position, gs: &HashMap<Position, isize>) -> isize {
        match gs.get(&node) {
            Some(g) => g + self.h(node),
            None => self.h(node)
        }
    }

    pub fn h(&self, node: Position) -> isize {
        return (node.x - self.end.x).abs() + (node.y - self.end.y).abs()
    }

    pub fn belt_successors(&self, node: Position) -> Vec<Position> {
        vec![
            node.forward(),
            node.forward().rotate_left(),
            node.forward().rotate_right(),
        ]
    }
}