use crate::Grid;
use rand::Rng;

extern crate alloc;
use alloc::vec::Vec;

#[test]
fn grid_basic() {
    let mut grid = Grid::<Vec<(f32, f32)>>::new(100.0, 100.0, 10, 10, false);
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
    let mut grid = Grid::<Vec<(f32, f32)>>::new(100.0, 100.0, 10, 10, true);
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
    let mut grid = Grid::<usize>::new(100.0, 100.0, 10, 10, false);
    for row in 0..10 {
        for col in 0..10 {
            let x = col as f32 * grid.cell_width;
            let y = row as f32 * grid.cell_height;
            if let Some(cell) = grid.get_cell_mut(x, y) {
                *cell = (row * 10) + col;
            };
        }
    }

    for (i, cell) in grid.iter_cells_in_rect(0.0, 0.0, 100.0, 100.0).enumerate() {
        assert_eq!(i, *cell);
    }
}

#[test]
fn iter_y_down() {
    let mut grid = Grid::<usize>::new(100.0, 100.0, 10, 10, false);
    for row in 0..10 {
        for col in 0..10 {
            let x = col as f32 * grid.cell_width;
            let y = (9 - row) as f32 * grid.cell_height;
            // print!("{}, {} -> ", x, y);
            if let Some(cell) = grid.get_cell_mut(x, y) {
                *cell = (row * 10) + col;
                // println!("{}", *cell);
            } else {
                // println!("None");
            }
        }
    }

    let iter = grid.iter_cells_in_rect(0.0, 0.0, 100.0, 100.0).y_down();
    // println!("{:#?}", iter);
    for (i, cell) in iter.enumerate() {
        // println!("{}", i);
        assert_eq!(i, *cell);
    }
}

#[test]
fn iter_coords() {
    let grid = Grid::<(usize, usize)>::new(100.0, 100.0, 10, 10, false);
    for (col, row) in grid.iter_coords_in_rect(25.0, 35.0, 65.0, 115.0) {
        // println!("{},{}", col, row);
        assert!(col > 1 && col < 7);
        assert!(row > 2 && row < 10);
    }

    // println!("y down...");
    let grid = Grid::<(usize, usize)>::new(100.0, 100.0, 10, 10, false);
    for (col, row) in grid.iter_coords_in_rect(25.0, 35.0, 65.0, 115.0).y_down() {
        // println!("{},{}", col, row);
        assert!(col > 1 && col < 7);
        assert!(row > 2 && row < 10);
    }
}

#[test]
fn grid_resize() {
    // Pivot at lower left corner
    let mut grid = Grid::<(usize, usize)>::new(100.0, 100.0, 10, 10, false);
    for col in 0..grid.columns() {
        for row in 0..grid.rows() {
            if let Some(cell) = grid.get_cell_by_indices_mut(col, row) {
                *cell = (col, row);
            };
        }
    }

    grid.set_size(1000.0, 200.0);
    assert_eq!(grid.width, 1000.0);
    assert_eq!(grid.height, 200.0);
    assert_eq!(grid.cell_width, 100.0);
    assert_eq!(grid.cell_height, 20.0);
    assert_eq!(grid.offset_x, 0.0);
    assert_eq!(grid.offset_y, 0.0);

    let iter = grid.iter_coords_in_rect(150.0, 50.0, 300.0, 150.0);
    for coords in iter {
        let value = grid.get_cell_by_indices(coords.0, coords.1);
        // println!("{:?} -> {:?}", coords, value );
        assert_eq!(Some(&coords), value);
    }

    // Centered Pivot
    let mut grid = Grid::<(usize, usize)>::new(100.0, 100.0, 10, 10, true);
    for col in 0..grid.columns() {
        for row in 0..grid.rows() {
            if let Some(cell) = grid.get_cell_by_indices_mut(col, row) {
                *cell = (col, row);
            };
        }
    }

    grid.set_size(1000.0, 200.0);
    assert_eq!(grid.width, 1000.0);
    assert_eq!(grid.height, 200.0);
    assert_eq!(grid.cell_width, 100.0);
    assert_eq!(grid.cell_height, 20.0);
    assert_eq!(grid.offset_x, 500.0);
    assert_eq!(grid.offset_y, 100.0);

    let iter = grid.iter_coords_in_rect(150.0, 50.0, 300.0, 150.0);
    for coords in iter {
        let value = grid.get_cell_by_indices(coords.0, coords.1);
        // println!("{:?} -> {:?}", coords, value );
        assert_eq!(Some(&coords), value);
    }
}

#[test]
fn outside_area(){
    let mut grid = Grid::<(usize, usize)>::new(100.0, 100.0, 10, 10, false);
    for col in 0..grid.columns() {
        for row in 0..grid.rows() {
            if let Some(cell) = grid.get_cell_by_indices_mut(col, row) {
                *cell = (col, row);
            };
        }
    }

    let a = grid.get_cell(-10.0, 20.0);
    assert_eq!(a, None);

    let b = grid.get_cell(10.0, 200.0);
    assert_eq!(b, None);

    let c = grid.get_cell(15.0, 15.0);
    assert_eq!(c, Some(&(1,1)));

    let mut iter = grid.iter_cells_in_rect(-25.0, -25.0, 5.0, 5.0);
    assert_eq!(iter.next(), Some(&(0,0))); // Only the left-bottom cell will be included
    assert_eq!(iter.next(), None);

    let mut iter = grid.iter_cells_in_rect(95.0, 95.0, 125.0, 125.0);
    assert_eq!(iter.next(), Some(&(9,9))); // Only the right-top cell will be included
    assert_eq!(iter.next(), None);
}
