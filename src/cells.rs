use crate::{ CELLS_COUNT, H, W };
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Cell {
    pub index: usize,
    pub value: u8,
    neighbor_indices: [Option<usize>; 8],
}

impl Cell {
    fn get_living_neighbor_count(&self, cells: &[Cell; CELLS_COUNT]) -> u8 {
        let mut count: u8 = 0;
        for neighbor_index in self.neighbor_indices {
            neighbor_index.inspect(|ni: &usize| {
                if cells[*ni].value == 0 {
                    return;
                }
                count += 1;
            });
        }
        count
    }

    fn populate_neighbor_indices(&mut self) {
        let mut neighbor_indices: [Option<usize>; 8] = [None; 8];
        let w_mod = self.index % W;
        let is_left_edge = w_mod == 0;
        let is_right_edge = w_mod == W - 1;
        let is_top_edge = self.index < W;
        let is_bottom_edge = self.index >= H * (W - 1);

        if !is_top_edge {
            neighbor_indices[1] = Some(self.index - W);
            if !is_left_edge {
                neighbor_indices[0] = Some(self.index - W - 1);
            }
            if !is_right_edge {
                neighbor_indices[2] = Some(self.index - W + 1);
            }
        }
        if !is_left_edge {
            neighbor_indices[3] = Some(self.index - 1);
        }
        if !is_right_edge {
            neighbor_indices[4] = Some(self.index + 1);
        }
        if !is_bottom_edge {
            neighbor_indices[6] = Some(self.index + W);
            if !is_left_edge {
                neighbor_indices[5] = Some(self.index + W - 1);
            }
            if !is_right_edge {
                neighbor_indices[7] = Some(self.index + W + 1);
            }
        }
        self.neighbor_indices = neighbor_indices;
    }
}

pub fn build_cells() -> [Cell; CELLS_COUNT] {
    let mut cells = [Cell { index: 0, value: 0, neighbor_indices: [Some(0); 8] }; CELLS_COUNT];
    let mut rng = rand::thread_rng();

    for ii in 0..CELLS_COUNT {
        cells[ii].index = ii;
        cells[ii].populate_neighbor_indices();
        cells[ii].value = rng.gen::<f64>().round() as u8;
    }

    cells
}

fn get_next_cell_value(cell: &Cell, cells: &[Cell; CELLS_COUNT]) -> u8 {
    let living_neighbors = cell.get_living_neighbor_count(&cells);
    if cell.value == 1 && living_neighbors < 2 {
        return 0;
    } else if cell.value == 1 && living_neighbors >= 4 {
        return 0;
    } else if cell.value == 0 && living_neighbors == 3 {
        return 1;
    }
    cell.value
}

pub fn get_next_cells(cells: &[Cell; CELLS_COUNT]) -> [Cell; CELLS_COUNT] {
    let mut next_cells = cells.clone();
    for ii in 0..CELLS_COUNT {
        next_cells[ii].value = get_next_cell_value(&cells[ii], &cells);
    }
    next_cells
}
