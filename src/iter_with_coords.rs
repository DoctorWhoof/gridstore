use super::*;

/// Iterator that yields (value, column, row) tuples from `IterGridRect`.
#[derive(Debug)]
pub struct IterWithCoords<'a, const COLS: usize, const ROWS: usize, V> {
    pub(super) iter: IterGridRect<'a, COLS, ROWS, V>,
    pub(super) current_col: usize,
    pub(super) current_row: usize,
}


impl<'a, const COLS: usize, const ROWS: usize, V> Iterator for IterWithCoords<'a, COLS, ROWS, V> {
    type Item = (&'a V, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iter.next() {
            // Capture the current coordinates
            let col = self.current_col;
            let row = self.current_row;

            // Advance the column, wrapping to the next row if needed
            self.current_col += 1;
            if self.current_col > self.iter.right {
                self.current_col = self.iter.left;
                self.current_row += 1;
            }

            Some((value, col, row))
        } else {
            None
        }
    }
}
