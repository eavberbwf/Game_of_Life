use ndarray::azip;
use ndarray::prelude::*;

use crate::type_impl::{States::*, *};

fn states_to_transitions_array(start: &States, end: &States) -> Array1<States> {
    match (start, end) {
        (Alive, Alive) => Array::from_elem(5, Alive),
        (Alive, Dead) => array![Alive, A1, A2, A3, Dead],
        (Dead, Alive) => array![Dead, D1, D2, D3, Alive],
        (Dead, Dead) => Array::from_elem(5, Dead),
        _ => Array::from_elem(5, Alive),
    }
}

pub fn grids_to_sequence_grid(grid1: &Grid<States>, grid2: &Grid<States>) -> Array3<States> {
    let mut time_sequence = Array::from_elem((5, SIZE, SIZE), Alive);
    azip!((index(i, j), &start in &grid1.matrix, &end in &grid2.matrix) {
        time_sequence.slice_mut(s![..,i,j]).assign(&states_to_transitions_array(&start, &end))
    });
    time_sequence
}

pub fn seq_to_grids(grid1: &Grid<States>, grid2: &Grid<States>) -> Array1<Grid<States>> {
    let sequence = grids_to_sequence_grid(grid1, grid2);
    sequence
        .outer_iter()
        .map(|time| Grid {
            matrix: time.to_owned(),
        })
        .collect()
}
