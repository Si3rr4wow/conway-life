use crate::{ H, W };
use std::thread;

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
    for ii in 0..H * W {
        counts[ii] = get_living_neighbor_count(&ii, &cells);
    }
    counts
}

fn get_next_cell_value(cell: u8, living_neighbor_count: u8) -> u8 {
    if cell == 1 && living_neighbor_count < 2 {
        return 0;
    } else if cell == 1 && living_neighbor_count >= 4 {
        return 0;
    } else if cell == 0 && living_neighbor_count == 3 {
        return 1;
    }
    cell
}

pub fn update_cells(mut cells: [u8; H * W]) -> [u8; H * W] {
    let living_neighbor_counts = get_living_neighbor_counts(&cells);
    let mut handles: Vec<thread::JoinHandle<(usize, u8)>> = Vec::new();
    let _ = thread::available_parallelism().inspect(|p| println!("{p}"));
    for ii in 0..H * W {
        let cell = cells[ii];
        let living_neighbor_count = living_neighbor_counts[ii];
        handles.push(
            thread::spawn(move || { (ii, get_next_cell_value(cell, living_neighbor_count)) })
        );
    }
    for handle in handles {
        // Note we're not handling the case of a thread panicking
        let _ = handle.join().inspect(|(ii, value)| {
            cells[*ii] = *value;
        });
    }
    cells
}
