use cell::Cell;
use crate::cell;
use crate::cell::ValueCell;

#[derive(Copy, Clone, Debug)]
pub struct Box {
    grid: [[Cell; 3]; 3]
}

impl Box {
    pub fn new() -> Self {
        let mut grid = [[Cell::new(0, 0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                grid[i][j] = Cell::new(i, j)
            }
        }
        Self {
            grid
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> ValueCell {
        self.grid[y][x].get_value()
    }
}