use crate::maze;
use crate::cell;

use maze::Maze;
use cell::{Cell, CellType};

use ::rand::thread_rng;
use ::rand::seq::SliceRandom;

pub struct RecursiveBacktrackingAlgorithm {
    pub maze: Maze,
    stack: Vec<(usize, usize)>,
    rng: ::rand::rngs::ThreadRng,
}

impl RecursiveBacktrackingAlgorithm {
    pub fn new(size: usize) -> Self {
        let mut maze = Maze::new(size);
        maze.set(0, 0, CellType::Start);
        RecursiveBacktrackingAlgorithm {
            maze,
            stack: vec![(0, 0)],
            rng: thread_rng(),
        }
    }

    pub fn step(&mut self) -> bool {
        if let Some(&(x, y)) = self.stack.last() {
            let mut neighbors = vec![];
            for (dx, dy) in &[(0, 2), (2, 0), (0, -2), (-2, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.maze.size as i32 && ny >= 0 && ny < self.maze.size as i32 
                   && self.maze.get(nx as usize, ny as usize).unwrap().cell_type == CellType::Wall {
                    neighbors.push((nx as usize, ny as usize));
                }
            }

            if !neighbors.is_empty() {
                let (nx, ny) = neighbors.choose(&mut self.rng).unwrap();
                self.maze.set((x + nx) / 2, (y + ny) / 2, CellType::Path);
                self.maze.set(*nx, *ny, CellType::Path);
                self.stack.push((*nx, *ny));
            } else {
                self.stack.pop();
            }
            true
        } else {
            self.maze.set(self.maze.size - 1, self.maze.size - 1, CellType::End);
            false
        }
    }
}