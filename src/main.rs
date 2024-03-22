mod cells;
use crate::cells::{print_cells, update_cells};
// use open;
use std::{thread, time};

const H: usize = 3;
const W: usize = 3;

/*
[tl][tt][tr]
[ll][ii][rr]
[bl][bb][br]
*/

/*
0 1 2
3 4 5
6 7 8
*/

fn main() {
    // let mut cells: [u8; H * W] = [0; H * W];
    // open::that("http://rust-lang.org"); // create a lil webpage to do io on
    let mut cells: [u8; H * W] = [0, 0, 0, 1, 1, 1, 0, 0, 0];

    loop {
        print_cells(&cells);
        println!("\n===");
        cells = update_cells(cells);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
