mod cells;
mod window;

use window::run;

const H: usize = 40;
const W: usize = 40;
const CELLS_COUNT: usize = H * W;

fn main() {
    run();
}
