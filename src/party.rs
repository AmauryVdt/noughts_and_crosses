use crate::grid::Grid;

pub struct Party {
    player1: String,
    player2: String,
    grid: Grid
}

impl Party {
    pub fn new() -> Self {
        Self {
            player1: "Player 1".to_string(),
            player2: "Player 2".to_string(),
            grid: Grid::new()
        }
    }

    pub fn start(&self) {
        let mut turn = 0;
        while self.grid.check_win() == ' ' {
            let symbol: char = if turn%2 == 0 { 'x' } else { 'o' };
            println!("Player {}, you are the {}:", turn%2+1, symbol);
            self.grid.play(symbol);
            turn += 1;
        }
    }
}
