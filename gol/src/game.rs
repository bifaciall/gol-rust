use crate::grid::Grid;

pub struct Game {
    pub grid: Grid,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let live_neighbors = self.live_neighbor_count(x, y);
                let cell = self.grid.get(x, y);
                let new_cell = match (cell, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
                new_grid.set(x, y, new_cell);
            }
        }
        self.grid = new_grid;
    }

    fn live_neighbor_count(&self, _x: usize, _y: usize) -> usize {
        // Implement neighbor count logic
        0
    }
}