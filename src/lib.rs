//! A Rectangular grid of non-overlapping rects containing a single generic item each.
//! Its dimensions can be centered around (0.0, 0.0) or start at the lower-left corner.
//! Once created, allows retrieving its contents via physical, f32 coordinates
//! or directly from colums/row/layer indices.

#![no_std]

use libm::floorf;

mod iter;
pub use iter::*;

mod iter_coords;
pub use iter_coords::*;

mod iter_with_coords;
pub use iter_with_coords::*;

#[cfg(test)]
mod test;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Grid<V> {
    // Dimensions
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    cell_width: f32,
    cell_height: f32,
    // Slots
    columns: usize,
    rows: usize,
    layers: usize,
    // Pivot. Values are from 0.0 to 1.0
    offset_x: f32,
    offset_y: f32,
    // Storage
    data: Vec<Vec<Vec<V>>>,
}

// Standard Error message helper with colors
macro_rules! err {
    ($msg:expr) => {
        concat!("\x1b[31m", "Grid Error: ", $msg, "\x1b[0m")
    };
}

// Default implementation
impl<V> Grid<V> where V: Default {}

impl<V> Clone for Grid<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            ..*self
        }
    }
}

// Unconstrained implementation.
impl<V> Grid<V> {
    /// Returns a Grid pre-filled with the result of function "func". It will
    /// be placed at (0.0, 0.0), with width and height set to 1.0.
    /// Use [resize] to adjust its physical size, and "[set_pivot]"
    /// if you want it centered around its position.
    pub fn new<F>(width:f32, height:f32, columns:usize, rows:usize, layers:usize, mut func: F) -> Self
    where
        F: FnMut() -> V,
    {
        let mut data: Vec<Vec<Vec<V>>> = Vec::new();
        for l in 0 .. layers {
            data.push(vec![]);
            for x in 0 .. columns {
                data[l].push(vec![]);
                for _y in 0 .. rows {
                    data[l][x].push(func());
                }
            }
        }
        assert!(width >= 0.0, err!("'width' must be > 0.0"));
        assert!(height >= 0.0, err!("'height' must be > 0.0"));
        assert!(columns > 0, err!("'columns' must be > 0"));
        assert!(rows > 0, err!("'rows' must be > 0"));
        assert!(layers > 0, err!("'layers' must be > 0"));
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
            cell_width: width / columns as f32,
            cell_height: height / columns as f32,
            columns,
            rows,
            layers,
            offset_x: width / 2.0,
            offset_y: width / 2.0,
            data,
        }
    }

    /// Sets the physical size
    pub fn resize(&mut self, width:f32, height:f32) {
        assert!(width >= 0.0, err!("'width' must be > 0.0"));
        assert!(height >= 0.0, err!("'height' must be > 0.0"));
        let current_offset_x = self.offset_x / self.width;
        let current_offset_y = self.offset_y / self.height;
        self.width = width;
        self.height = height;
        self.cell_width = width / self.columns as f32;
        self.cell_height = height / self.rows as f32;
        self.offset_x = current_offset_x * width;
        self.offset_y = current_offset_y * height;
    }

    /// Sets the pivot point, in a range from 0.0 (min) to 1.0 (max),
    /// with 0.5 being centered
    pub fn set_pivot(&mut self, x:f32, y:f32) {
        self.offset_x = self.width * x;
        self.offset_y = self.height * y;
    }

    /// Physical width.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Physical height.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Layer count.
    pub fn layers(&self) -> usize {
        self.layers
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
        self.columns
    }

    /// Total number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// The left-most edge occupied by the Grid. This is the Y origin if grid is not centered.
    pub fn left(&self) -> f32 {
        self.x - self.offset_x
    }

    /// The right-most edge occupied by the Grid.
    pub fn right(&self) -> f32 {
        self.x + self.width - self.offset_x
    }

    /// The bottom-most edge occupied by the Grid. WARNING, coordinates are Y up
    /// (positive values go up), so this is the Y origin if the grid is not centered.
    pub fn bottom(&self) -> f32 {
        self.y - self.offset_y
    }

    /// The top-most edge occupied by the Grid. WARNING, coordinates are Y up (positive values go up).
    pub fn top(&self) -> f32 {
        self.y + self.height - self.offset_y
    }

    /// The horizontal offset if the center is not at (0.0, 0.0)
    pub fn offset_x(&self) -> f32 {
        self.offset_x
    }

    /// The vertical offset if the center is not at (0.0, 0.0)
    pub fn offset_y(&self) -> f32 {
        self.offset_y
    }

    /// Returns an optional tuple with the current coordinates in the (column, row) format, given
    /// x and y "physical" coordinates.
    pub fn get_cell_coords(&self, x: f32, y: f32) -> Option<(usize, usize)> {
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
        Some((col, row))
    }

    /// Returns an optional reference to the content of a cell containing the
    /// provided coordinates, if any.
    pub fn get_cell(&self, x: f32, y: f32, layer: usize) -> Option<&V> {
        let coords = self.get_cell_coords(x, y)?;
        self.get_cell_by_indices(coords.0, coords.1, layer)
    }

    /// Returns an optional mutable reference to the content of a cell containing the
    /// provided coordinates, if any.
    pub fn get_cell_mut(&mut self, x: f32, y: f32, layer: usize) -> Option<&mut V> {
        let coords = self.get_cell_coords(x, y)?;
        self.get_cell_by_indices_mut(coords.0, coords.1, layer)
    }

    /// Returns an optional reference to the content of a cell in the
    /// provided coordinates, if any.
    pub fn get_cell_by_indices(&self, col: usize, row: usize, layer: usize) -> Option<&V> {
        let layer = self.data.get(layer)?;
        let col = layer.get(col)?;
        let cell = col.get(row)?;
        Some(cell)
    }

    /// Returns an optional mutable reference to the content of a cell in the
    /// provided coordinates, if any.
    pub fn get_cell_by_indices_mut(
        &mut self,
        col: usize,
        row: usize,
        layer: usize,
    ) -> Option<&mut V> {
        let layer = self.data.get_mut(layer)?;
        let col = layer.get_mut(col)?;
        let cell = col.get_mut(row)?;
        Some(cell)
    }

    fn get_edges(
        &self,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> (usize, usize, usize, usize) {
        // Apply offsets
        let left = left + self.offset_x;
        let bottom = bottom + self.offset_y;
        let right = right + self.offset_x;
        let top = top + self.offset_y;
        // Get columns and rows
        let max_right = self.columns() - 1;
        let col_left = (floorf(left / self.cell_width).max(0.0) as usize).min(max_right);
        let col_right = (floorf(right / self.cell_width) as usize).min(max_right);

        let max_top = self.rows() - 1;
        let row_bottom = (floorf(bottom / self.cell_height).max(0.0) as usize).min(max_top);
        let row_top = (floorf(top / self.cell_height) as usize).min(max_top);
        (col_left, row_bottom, col_right, row_top)
    }

    /// Returns an iterator with the cells overlapping a rectangle, starting at the
    /// bottom/left corner and moving all the way to the top/right corner if y_up is "true",
    /// and from top to bottom if y_up is "false".
    pub fn iter_cells_in_rect(
        &self,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
        layer: usize,
    ) -> IterGridRect<'_, V> {
        let (col_left, row_bottom, col_right, row_top) = self.get_edges(left, bottom, right, top);
        // Create and return the iterator with calculated bounds
        // println!("{}, {} -> {}, {}", col_left, row_bottom, col_right, row_top);
        IterGridRect {
            layer,
            y_up: true,
            grid: self,
            left: col_left,
            right: col_right,
            top: row_top,
            bottom: row_bottom,
            current_row: row_bottom,
            current_col: col_left,
            done: false,
        }
    }

    /// Returns an iterator that yields (column,row) pairs for each cell that overlaps the provided
    /// rectangle edges.
    pub fn iter_coords_in_rect(&self, left: f32, bottom: f32, right: f32, top: f32) -> IterCoords {
        let (col_left, row_bottom, col_right, row_top) = self.get_edges(left, bottom, right, top);
        IterCoords {
            y_up: true,
            top: row_top,
            bottom: row_bottom,
            left: col_left,
            right: col_right,
            current_row: row_bottom,
            current_col: col_left,
            done: false,
        }
    }

    /// Returns an iterator that yields (column,row) pairs for each cell that overlaps the provided
    /// rectangle edges.
    pub fn iter_info_in_rect(&self, left: f32, bottom: f32, right: f32, top: f32) -> IterCoords {
        let (col_left, row_bottom, col_right, row_top) = self.get_edges(left, bottom, right, top);
        IterCoords {
            y_up: true,
            top: row_top,
            bottom: row_bottom,
            left: col_left,
            right: col_right,
            current_row: row_bottom,
            current_col: col_left,
            done: false,
        }
    }

    /// Returns an iterator with all cells in all layers.
    pub fn iter_all_cells(
        &self,
    ) -> core::iter::Flatten<core::iter::Flatten<core::slice::Iter<'_, Vec<Vec<V>>>>> {
        self.data.iter().flatten().flatten()
    }

    /// Allows a single function to modify the contents of all cells.
    /// The function will take a mutable reference to the cell contents
    pub fn modify_all<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut V),
    {
        for layer in &mut self.data {
            for col in layer {
                for cell in col {
                    func(cell)
                }
            }
        }
    }

    /// Allows a closure to modify the contents of any cell that overlaps a given rectangle.
    /// The closure's arguments are "coords:(usize, usize)", "value:&mut V"
    pub fn modify_in_rect<F>(
        &mut self,
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
        layer: usize,
        mut func: F,
    ) where
        F: FnMut((usize, usize), &mut V),
    {
        for coords in self.iter_coords_in_rect(left, bottom, right, top) {
            let Some(cell) = self.get_cell_by_indices_mut(coords.0, coords.1, layer) else {
                continue;
            };
            func(coords, cell);
        }
    }

    /// Returns a reference to the underlying data.
    pub fn raw_data(&self) -> &Vec<Vec<Vec<V>>> {
        &self.data
    }

    /// Returns a reference to the underlying data. Be careful and don't resize it!
    pub fn raw_data_mut(&mut self) -> &mut Vec<Vec<Vec<V>>> {
        &mut self.data
    }
}
