//! A Rectangular grid of non-overlapping rects containing a single generic item each.
//! Its dimensions can be centered around (0.0, 0.0) or start at the lower-left corner.
//! Once created, allows retrieving its contents via physical, f32 coordinates
//! or directly from colums/row indices.

#![no_std]

#[derive(Debug)]
pub struct Grid<const COLS: usize, const ROWS: usize, V> {
    // Dimensions
    width: f32,
    height: f32,
    cell_width: f32,
    cell_height: f32,
    //Pivot
    offset_x: f32,
    offset_y: f32,
    // Storage
    data: [[V; ROWS]; COLS],
}

// Standard Error message helper
macro_rules! err {
    ($msg:expr) => {
        concat!("\x1b[31m", "Grid Error: ", $msg, "\x1b[0m")
    };
}

// Default implementation always needs "width" and "height" provided.
impl<const COLS: usize, const ROWS: usize, V> Grid<COLS, ROWS, V>
where
    V: Default,
{
    pub fn new(width: f32, height: f32, centered: bool) -> Self {
        Self::new_with(width, height, centered, || Default::default())
    }
}

// Unconstrained implementation.
impl<const COLS: usize, const ROWS: usize, V> Grid<COLS, ROWS, V> {
    /// Returns a Grid pre-filled with the result of function "func"
    pub fn new_with<F>(width: f32, height: f32, centered: bool, mut func: F) -> Self
    where
        F: FnMut() -> V,
    {
        use core::array::from_fn;
        assert!(width >= 0.0, err!("Width must be positive"));
        assert!(height >= 0.0, err!("Height must be positive"));
        let cell_width = width / COLS as f32;
        let cell_height = height / ROWS as f32;

        Self {
            width,
            height,
            cell_width,
            cell_height,
            offset_x: if centered { width / 2.0 } else { 0.0 },
            offset_y: if centered { height / 2.0 } else { 0.0 },
            data: from_fn(|_col| from_fn(|_row| func())),
        }
    }

    /// Physical width.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Physical height.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Physical width of each cell.
    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    /// Physical height of each cell.
    pub fn cell_height(&self) -> f32 {
        self.cell_height
    }

    /// Total number of columns.
    pub fn columns(&self) -> usize {
        COLS
    }

    /// Total number of rows.
    pub fn rows(&self) -> usize {
        ROWS
    }

    /// The left-most edge occupied by the Grid. This is the Y origin if grid is not centered.
    pub fn left(&self) -> f32 {
        -self.offset_x
    }

    /// The right-most edge occupied by the Grid.
    pub fn right(&self) -> f32 {
        self.width - self.offset_x
    }

    /// The bottom-most edge occupied by the Grid. WARNING, coordinates are Y up
    /// (positive values go up), so this is the Y origin if the grid is not centered.
    pub fn bottom(&self) -> f32 {
        -self.offset_y
    }

    /// The top-most edge occupied by the Grid. WARNING, coordinates are Y up (positive values go up).
    pub fn top(&self) -> f32 {
        self.height - self.offset_x
    }


    /// Returns an optional reference to the content of a cell containing the
    /// provided coordinates, if any.
    pub fn get_cell(&self, x: f32, y: f32) -> Option<&V> {
        let x = x + self.offset_x;
        if x < 0.0 {
            return None;
        }
        let y = y + self.offset_y;
        if y < 0.0 {
            return None;
        }
        let col = libm::floorf(x / self.cell_width) as usize;
        let row = libm::floorf(y / self.cell_height) as usize;
        self.get_cell_by_indices(col, row)
    }

    /// Returns an optional mutable reference to the content of a cell containing the
    /// provided coordinates, if any.
    pub fn get_cell_mut(&mut self, x: f32, y: f32) -> Option<&mut V> {
        let x = x + self.offset_x;
        if x < 0.0 {
            return None;
        }
        let y = y + self.offset_y;
        if y < 0.0 {
            return None;
        }
        let col = libm::floorf(x / self.cell_width) as usize;
        let row = libm::floorf(y / self.cell_height) as usize;
        self.get_cell_by_indices_mut(col, row)
    }

    /// Returns an optional reference to the content of a cell in the
    /// provided coordinates, if any.
    pub fn get_cell_by_indices(&self, col: usize, row: usize) -> Option<&V> {
        let col = self.data.get(col)?;
        let cell = col.get(row)?;
        Some(cell)
    }

    /// Returns an optional mutable reference to the content of a cell in the
    /// provided coordinates, if any.
    pub fn get_cell_by_indices_mut(&mut self, col: usize, row: usize) -> Option<&mut V> {
        let col = self.data.get_mut(col)?;
        let cell = col.get_mut(row)?;
        Some(cell)
    }

    // /// Allows a single function to modify the contents of all cells.
    // /// The function will take a mutable reference to the cell contents, the current
    // /// Column index and the current Row index.
    // pub fn modify_all<F>(&mut self, mut func: F)
    // where
    //     F: FnMut(&mut V, usize, usize),
    // {
    //     for (col_index, col) in &mut self.data.iter_mut().enumerate() {
    //         for (row_index, cell) in col.iter_mut().enumerate() {
    //             func(cell, col_index, row_index)
    //         }
    //     }
    // }

    /// Allows a single function to modify the contents of all cells.
    /// The function will take a mutable reference to the cell contents
    pub fn modify_all<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut V),
    {
        for col in &mut self.data {
            for cell in col {
                func(cell)
            }
        }
    }

    /// Allows a function to modify the contents of any cell that overlaps a rectangle.
    /// Rectangles are only allowed to be as big as the cell size. Bigger rects will
    /// fail an assert in Debug builds, and do nothing in release builds. This may change to a result
    /// in the future.
    pub fn modify_in_rect<F>(&mut self, top: f32, left: f32, bottom: f32, right: f32, mut func: F)
    where
        F: FnMut(&mut V),
    {
        // Apply offsets
        let left = left + self.offset_x;
        let right = right + self.offset_x;
        let top = top + self.offset_y;
        let bottom = bottom + self.offset_y;
        // Validate
        if !self.validate_rect(top, left, bottom, right) {
            return;
        };
        // Get columns and ROWS
        let col_1 = (left / self.cell_width) as usize;
        let col_2 = (right / self.cell_width) as usize;
        let row_1 = (top / self.cell_height) as usize;
        let row_2 = (bottom / self.cell_height) as usize;
        // Modify (if needed)!
        if row_1 != row_2 {
            let value = &mut self.data[col_1][row_2];
            func(value);
        }
        if col_1 != col_2 {
            let value = &mut self.data[col_2][row_1];
            func(value);
            if row_1 != row_2 {
                let value = &mut self.data[col_2][row_2];
                func(value);
            }
        }

        let value = &mut self.data[col_1][row_1];
        func(value);
    }

    /// Returns a reference to the underlying data. Be careful!
    pub fn raw_data(&mut self) -> &mut [[V; ROWS]; COLS] {
        &mut self.data
    }

    #[inline(always)]
    fn validate_rect(&self, top: f32, left: f32, bottom: f32, right: f32) -> bool {
        let w = right - left;
        let h = top - bottom;
        // In Debug build an invalid rect is an error.
        // TODO: May be removed later, always return result instead
        #[cfg(debug_assertions)]
        {
            assert!(w >= 0.0, err!("rect width must be positive"));
            assert!(
                w < self.cell_width,
                err!("rect width larger than cell width")
            );
            assert!(h >= 0.0, err!("rect height must be positive"));
            assert!(
                h < self.cell_height,
                err!("rect height larger than cell height")
            );
        }
        // In Release build an invalid rect returns false
        if w < 0.0 || w > self.cell_width {
            return false;
        }
        if h < 0.0 || h > self.cell_height {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::Grid;
    use rand::Rng;

    extern crate alloc;
    use alloc::vec::Vec;

    #[test]
    fn grid_basic() {
        let mut grid = Grid::<10, 10, Vec<(f32, f32)>>::new(100.0, 100.0, false);
        let mut rng = rand::thread_rng();
        for _n in 0..100 {
            let x = rng.gen_range(0.0..100.0);
            let y = rng.gen_range(0.0..100.0);
            if let Some(container) = grid.get_cell_mut(x, y) {
                container.push((x, y));
            };
        }

        for (i_x, col) in grid.data.iter().enumerate() {
            for (i_y, cell) in col.iter().enumerate() {
                if cell.is_empty() {
                    continue;
                }
                for value in cell {
                    assert_eq!((value.0 / grid.cell_width).floor() as usize, i_x);
                    assert_eq!((value.1 / grid.cell_height).floor() as usize, i_y);
                }
                // println!("{},{} -> {:.1?}", i_x, i_y, cell.data)
            }
        }
    }

    #[test]
    fn grid_negative_values() {
        let mut grid = Grid::<10, 10, Vec<(f32, f32)>>::new(100.0, 100.0, true);
        let mut rng = rand::thread_rng();
        for _n in 0..100 {
            let x = rng.gen_range(grid.left()..grid.right());
            let y = rng.gen_range(grid.bottom()..grid.top());
            if let Some(container) = grid.get_cell_mut(x, y) {
                container.push((x, y));
            };
        }

        for (i_x, col) in grid.data.iter().enumerate() {
            for (i_y, cell) in col.iter().enumerate() {
                if cell.is_empty() {
                    continue;
                }
                // println!("{},{} -> {:.1?}", i_x, i_y, cell);
                for value in cell {
                    let col = ((value.0 + grid.offset_x) / grid.cell_width).floor() as usize;
                    let row = ((value.1 + grid.offset_y) / grid.cell_height).floor() as usize;
                    assert_eq!(col, i_x);
                    assert_eq!(row, i_y);
                }
            }
        }
    }
}
