use crate::grid::{Grid, Value};
use std::io;

struct Party {
    player1: Value,
    player2: Value,
    grid: Grid
}

impl Party {
    pub fn new() -> Self {
        Self {
            player1: Value::CROSS,
            player2: Value::CIRCLE,
            grid: Grid::new()
        }
    }



}
