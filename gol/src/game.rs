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
    pub fn randomize_grid(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let random_value: bool = rng.gen();
                self.grid.set(x, y, random_value);
            }
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

    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1{
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx < self.grid.width && ny < self.grid.height && self.grid.get(nx, ny) {
                count += 1;
            }
            }
        } 
        count
    }
}