use std::io;
use std::io::Read;
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

    fn set_value(&mut self, value: char, x: usize, y: usize) {
        self.grid[y][x] = value;
    }

     fn display_grid(&self) {
         for line in self.grid.iter() {
             println!(" {} | {} | {}", line[0], line[1], line[2]);
         }
     }

    pub fn play(&self, symbol: char) {
        loop {
            self.display_grid();
            let mut buffer_x = String::new();
            let mut buffer_y = String::new();

            println!("Chose where to play. Enter a number for x: ");
            io::stdin().read_line(&mut buffer_x).unwrap();
            println!("Enter a number for y: ");
            io::stdin().read_line(&mut buffer_y).unwrap();

            let x: usize = match buffer_x.trim().parse() {
                Ok(x) => x,
                _ => { println!("'x' is not a number"); continue }
            };
            let y: usize = match buffer_y.trim().parse() {
                Ok(x) => x,
                _ => { println!("'y' is not a number"); continue }
            };

            if x < 0 || x > 2 || y < 0 || y < 2 {
                println!("The point is out of range");
                continue;
            }
            if self.grid[y][x] != ' ' {
                println!("This point is already played by a player");
                continue;
            }
            println!("You choose the point ({}, {})", x, y);
            // &self.grid[y][x] = &symbol;
            // self.set_value(symbol, x, y);
            break;
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
        } else if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[0][2] != ' ' {
            is_won= grid[0][2];
        }
        // Check for no win
        let mut is_full = true;
        for line in grid.iter() {
            for point in line {
                if point == &' ' {
                    is_full = false
                }
            }
        }
        if is_full { is_won = '*' }
        is_won
    }
}