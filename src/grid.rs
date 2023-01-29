use std::string::ToString;

#[derive(Copy, Clone, Debug)]
pub struct Grid {
    grid: [[char; 3]; 3]
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = [[' '; 3]; 3];
        Self {
            grid
        }
    }
     pub fn display_grid(&self) {
         for line in self.grid.iter() {
             println!(" {} | {} | {}", line[0], line[1], line[2])
         }
     }

    pub fn check_win(&self) -> char  {
        let mut is_won = ' ';
        let grid = self.grid;
        // Check for line
        for line in grid.iter() {
            if line[0] == line[1] && line[1] == line[2] && line[0] != ' ' {
                is_won = line[0]
            }
            // println!("{:?}", y)
        }
        // Check for column
        for col in 0..3 {
            if grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col] && grid[0][col] != ' ' {
                is_won = grid[0][col]
            }
        }
        // Check for diagonal
        if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[0][0] != ' ' {
            is_won= grid[0][0];
        } else if plateau[0][2] == plateau[1][1] && plateau[1][1] == plateau[2][0] && plateau[0][2] != ' ' {
            is_won= grid[0][2];
        }
        is_won
    }
}