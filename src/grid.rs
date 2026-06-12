pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Cell>,
}
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub value: Option<char>,
    pub color: Option<Color>,
}

#[derive(PartialEq)]
pub enum Color {
    Green,
    Yellow,
    Gray,
}

impl Grid {
    pub fn new() -> Self {
        let mut cells = Vec::new();
        for row in 0..6 {
            for col in 0..5 {
                cells.push(Cell {
                    row,
                    col,
                    value: None,
                    color: None,
                });
            }
        }
        Self {
            rows: 6,
            cols: 5,
            cells,
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row * self.cols + col)
    }
    pub fn populate_cell(&mut self, row: usize, col: usize, value: Option<char>, color: Option<Color>) {
        if let Some(cell) = self.cells.get_mut(row * self.cols + col) {
            cell.value = value;
            cell.color = color;
        }
    }
    
}
