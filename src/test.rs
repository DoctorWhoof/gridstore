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

#[test]
fn iter_y_up() {
    let mut grid = Grid::<10, 10, usize>::new(100.0, 100.0, false);
    for row in 0..10 {
        for col in 0..10 {
            let x = col as f32 * grid.cell_width;
            let y = row as f32 * grid.cell_height;
            if let Some(cell) = grid.get_cell_mut(x, y) {
                *cell = (row * 10) + col;
            };
        }
    }

    for (i, cell) in grid.iter_cells_in_rect(0.0, 0.0, 100.0, 100.0, true).enumerate(){
        assert_eq!(i, *cell);
    }
}

#[test]
fn iter_y_down() {
    let mut grid = Grid::<10, 10, usize>::new(100.0, 100.0, false);
    for row in 0..10 {
        for col in 0..10 {
            let x = col as f32 * grid.cell_width;
            let y = (9 - row) as f32 * grid.cell_height;
            if let Some(cell) = grid.get_cell_mut(x, y) {
                *cell = (row * 10) + col;
            };
        }
    }

    let iter = grid.iter_cells_in_rect(0.0, 0.0, 100.0, 100.0, false);
    // println!("{:#?}", iter);
    for (i, cell) in iter.enumerate(){
        // println!("{}", i);
        assert_eq!(i, *cell);
    }
}
