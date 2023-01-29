use std::string::ToString;

#[derive(Copy, Clone, Debug)]
pub struct Grid {
    grid: [[Value; 3]; 3]
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = [[Value::NONE; 3]; 3];
        Self {
            grid
        }
    }
     pub fn display_grid(&self) {
         for i in 0..3 {
             let mut line_y = String::new();
             for j in 0..3 {
                 line_y.push_str(match self.grid[i][j] {
                     Value::CIRCLE => "o",
                     Value::CROSS => "x",
                     _ => " "
                 });
                 if j < 2 { line_y.push_str("|"); }
             }
             println!("{}", line_y);
         }
     }

    pub fn check_win(&self)  {
        let mut is_won = false;
        for y in self.grid[1..].iter() {
            dbg!(y)
            // if self.grid[y] { }
            // for x in self.grid[y].len() {
            //
            // }
        }
        println!("{}", is_won)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    // #[default]
    NONE,
    CROSS,
    CIRCLE
}