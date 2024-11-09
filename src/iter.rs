use super::*;

/// Iterator that yields references to cells in the grid overlapping with a specified rectangle.
#[derive(Debug)]
pub struct IterGridRect<'a, const COLS: usize, const ROWS: usize, V> {
    pub(super) y_up: bool,
    pub(super) grid: &'a Grid<COLS, ROWS, V>,
    pub(super) top: usize,
    pub(super) bottom: usize,
    pub(super) left: usize,
    pub(super) right: usize,
    pub(super) current_row: usize,
    pub(super) current_col: usize,
    pub(super) done: bool,
}

impl<'a, const COLS: usize, const ROWS: usize, V> Iterator for IterGridRect<'a, COLS, ROWS, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.done == true {
                break;
            }
            if let Some(col) = self.grid.data.get(self.current_col) {
                if let Some(cell) = col.get(self.current_row) {
                    self.advance();
                    return Some(cell);
                } else {
                    break;
                }
            } else {
                self.advance();
            }
        }
        None
    }
}

impl<'a, const COLS: usize, const ROWS: usize, V> IterGridRect<'a, COLS, ROWS, V> {
    /// Returns an iterator that enumerates each cell with its coordinates (value, column, row).
    pub fn enumerate_coords(self) -> IterWithCoords<'a, COLS, ROWS, V> {
        let current_col = self.current_col;
        let current_row = self.current_row;
        IterWithCoords {
            iter: self,
            current_col,
            current_row,
        }
    }

    pub fn advance(&mut self) {
        // Advance column
        self.current_col += 1;
        // Wrap around to the next row if necessary
        if self.current_col > self.right {
            self.current_col = self.left;
            if self.y_up {
                self.current_row += 1;
                if self.current_row > self.top {
                    self.done = true;
                }
            } else {
                if self.current_row == self.bottom {
                    self.done = true;
                } else {
                    self.current_row -= 1;
                }
            }
        }
    }
}
