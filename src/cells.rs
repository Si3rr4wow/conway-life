use crate::{H, W};

fn get_neighbor_indices(&index: &usize) -> [Option<usize>; 8] {
    let mut neighbors: [Option<usize>; 8] = [None; 8];
    let w_mod = index % W;
    let is_left_edge = w_mod == 0;
    let is_right_edge = w_mod == W - 1;
    let is_top_edge = index < W;
    let is_bottom_edge = index >= H * (W - 1);

    if !is_top_edge {
        neighbors[1] = Some(index - W); //tt
        if !is_left_edge {
            neighbors[0] = Some(index - W - 1); //tl
        }
        if !is_right_edge {
            neighbors[2] = Some(index - W + 1); //tr
        }
    }
    if !is_left_edge {
        neighbors[3] = Some(index - 1); //ll
    }
    if !is_right_edge {
        neighbors[4] = Some(index + 1); //rr
    }
    if !is_bottom_edge {
        neighbors[6] = Some(index + W); //bb
        if !is_left_edge {
            neighbors[5] = Some(index + W - 1); //bl
        }
        if !is_right_edge {
            neighbors[7] = Some(index + W + 1); //br
        }
    }
    neighbors
}

fn get_living_neighbor_count(&index: &usize, &cells: &[u8; H * W]) -> u8 {
    let neighbor_indices = get_neighbor_indices(&index);
    let mut count: u8 = 0;
    for neighbor_index in neighbor_indices {
        neighbor_index.inspect(|ni: &usize| {
            if cells[*ni] == 0 {
                return;
            }
            count += 1;
        });
    }
    count
}

fn get_living_neighbor_counts(&cells: &[u8; H * W]) -> [u8; H * W] {
    let mut counts: [u8; H * W] = [0; H * W];
    for ii in 0..(H * W) {
        counts[ii] = get_living_neighbor_count(&ii, &cells);
    }
    counts
}

pub fn update_cells(mut cells: [u8; H * W]) -> [u8; H * W] {
    let living_neighbor_counts = get_living_neighbor_counts(&cells);
    for ii in 0..(H * W) {
        if cells[ii] == 1 && living_neighbor_counts[ii] < 2 {
            cells[ii] = 0;
        } else if cells[ii] == 1 && living_neighbor_counts[ii] < 4 {
            continue;
        } else if cells[ii] == 1 && living_neighbor_counts[ii] >= 4 {
            cells[ii] = 0;
        } else if cells[ii] == 0 && living_neighbor_counts[ii] == 3 {
            cells[ii] = 1;
        }
    }
    cells
}

pub fn print_cells(&cells: &[u8; H * W]) {
    for entry in cells.iter().enumerate() {
        let (index, value) = entry;
        if index % W == 0 {
            print!("\n")
        }
        print!("{value}")
    }
}
