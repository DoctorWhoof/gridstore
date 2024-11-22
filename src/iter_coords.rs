/// Iterator that yields (column,row) pairs for each cell that overlaps the provided
/// rectangle edges.
#[derive(Debug, Clone)]
pub struct IterCoords {
    pub(super) y_up: bool,
    pub(super) top: usize,
    pub(super) bottom: usize,
    pub(super) left: usize,
    pub(super) right: usize,
    pub(super) current_row: usize,
    pub(super) current_col: usize,
    pub(super) done: bool,
}

impl Iterator for IterCoords {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.done == true {
                break;
            }
            let result = (self.current_col, self.current_row);
            self.advance();
            return Some(result);
        }
        None
    }
}

impl IterCoords {
    pub fn y_down(self) -> Self {
        assert_eq!(
            self.current_row, self.bottom,
            "IterCoords: Error, 'y_down()' can only be used on freshly created Iterator."
        );
        let top = self.top;
        Self {
            y_up: false,
            current_row: top,
            ..self
        }
    }

    fn advance(&mut self) {
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
