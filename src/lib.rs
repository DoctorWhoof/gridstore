// #![no_std]

#[derive(Debug)]
pub struct Grid<V> {
    // Dimensions
    width: f32,
    height: f32,
    cell_width: f32,
    cell_height: f32,
    //Pivot
    offset_x: f32,
    offset_y: f32,
    // Storage
    data: Vec<Vec<V>>,
}

// Standard Error message helper
#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        concat!("\x1b[31m", "Grid Error: ", $msg, "\x1b[0m")
    };
}

/// A Rectangular grid of non-overlapping rects containing a single item each. Can be centered
/// around (0.0, 0.0) or start at the lower-left corner.
impl<V> Grid<V>
where
    V: Default,
{
    pub fn new(cols: usize, rows: usize, width: f32, height: f32, centered: bool) -> Self {
        Self::new_with(cols, rows, width, height, centered, || Default::default())
    }
}

impl<V> Grid<V> {
    pub fn new_with<F>(
        cols: usize,
        rows: usize,
        width: f32,
        height: f32,
        centered: bool,
        mut func: F,
    ) -> Self
    where
        F: FnMut() -> V,
    {
        assert!(width >= 0.0, err!("size width must be positive"));
        assert!(height >= 0.0, err!("size height must be positive"));
        let cell_width = width / cols as f32;
        let cell_height = height / rows as f32;

        Self {
            width,
            height,
            cell_width,
            cell_height,
            offset_x: if centered { width / 2.0 } else { 0.0 },
            offset_y: if centered { height / 2.0 } else { 0.0 },
            data: (0..cols)
                .map(|_x| (0..rows).map(|_y| func()).collect())
                .collect(),
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    pub fn cell_height(&self) -> f32 {
        self.cell_height
    }

    pub fn columns(&self) -> usize {
        self.data.len()
    }

    pub fn rows(&self) -> usize {
        if self.data.is_empty() {
            return 0;
        }
        self.data[0].len()
    }

    pub fn left(&self) -> f32 {
        -self.offset_x
    }

    pub fn right(&self) -> f32 {
        self.width - self.offset_x
    }

    pub fn bottom(&self) -> f32 {
        -self.offset_y
    }

    pub fn top(&self) -> f32 {
        self.height - self.offset_x
    }

    /// Returns an optional mutable reference to the content of a cell containing the
    /// provided coordinates, if any.
    pub fn get_coords_mut(&mut self, x: f32, y: f32) -> Option<&mut V> {
        let x = x + self.offset_x;
        let y = y + self.offset_y;
        #[cfg(debug_assertions)]
        {
            assert!(x >= 0.0, err!("Coordinate X can't be negative"));
            assert!(y >= 0.0, err!("Coordinate Y can't be negative"));
        }
        if x < 0.0 {
            return None;
        }
        if y < 0.0 {
            return None;
        }
        let col = (x / self.cell_width).floor() as usize;
        let row = (y / self.cell_height).floor() as usize;
        self.get_cell_mut(col, row)
    }

    /// Returns an optional mutable reference to the content of a cell containing the
    /// provided coordinates, if any.
    pub fn get_cell_mut(&mut self, col: usize, row: usize) -> Option<&mut V> {
        let col = self.data.get_mut(col)?;
        let cell = col.get_mut(row)?;
        Some(cell)
    }

    /// Allows a function to modify the contents of all cells with the same function.
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
    /// fail an assert in Debug builds, and do nothing in release builds.
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
        // Get columns and rows
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

    /// Returns a reference to the undelying data. Be careful! Changing the length
    /// of the vectors will break things!
    pub fn raw_data(&mut self) -> &mut Vec<Vec<V>> {
        &mut self.data
    }

    #[inline(always)]
    fn validate_rect(&self, top: f32, left: f32, bottom: f32, right: f32) -> bool {
        let w = right - left;
        let h = top - bottom;
        // In Debug build an invalid rect is an error
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
    #[test]
    fn grid_basic() {
        let mut grid: Grid<Vec<(f32, f32)>> = Grid::new(10, 10, 100.0, 100.0, false);
        let mut rng = rand::thread_rng();
        for _n in 0..100 {
            let x = rng.gen_range(0.0..100.0);
            let y = rng.gen_range(0.0..100.0);
            if let Some(container) = grid.get_coords_mut(x, y) {
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
        let mut grid: Grid<Vec<(f32, f32)>> = Grid::new(10, 10, 100.0, 100.0, true);
        let mut rng = rand::thread_rng();
        for _n in 0..100 {
            let x = rng.gen_range(grid.left()..grid.right());
            let y = rng.gen_range(grid.bottom()..grid.top());
            if let Some(container) = grid.get_coords_mut(x, y) {
                container.push((x, y));
            };
        }

        for (i_x, col) in grid.data.iter().enumerate() {
            for (i_y, cell) in col.iter().enumerate() {
                if cell.is_empty() {
                    continue;
                }
                println!("{},{} -> {:.1?}", i_x, i_y, cell);
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
