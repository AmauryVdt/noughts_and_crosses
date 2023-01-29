#[derive(Copy, Clone, Debug)]
pub struct Cell {
    x: usize,
    y: usize,
    value: ValueCell
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            value: ValueCell::NONE
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_value(&self) -> ValueCell {
        self.value
    }

    pub fn set_value(&mut self, value: ValueCell) {
        self.value = value;
    }

    pub fn print_position(&self) {
        println!("The cell is at x: {}, y: {}", self.x, self.y.to_string())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ValueCell {
    NONE,
    CROSS,
    CIRCLE
}