mod grid;
mod party;

use grid::Grid;

fn main() {
    println!("Hello, world!");
    let grid = Grid::new();
    grid.display_grid();
    grid.check_win();
}