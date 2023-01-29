use crate::grid::{Grid, Value};
use std::io;

struct Party {
    player1: String,
    player2: String,
    grid: Grid
}

impl Party {
    pub fn new() -> Self {
        Self {
            player1: "Player 1",
            player2: "Player 2",
            grid: Grid::new()
        }
    }
}
