mod type_impl;
use crate::type_impl::{Grid, States, States::*, SIZE};
use ndarray::azip;
use ndarray::prelude::*;

mod sequence_gen;

fn main() {
    let g: Grid<States> = Grid::new();
    println!("{}", g);
    println!("{}", g.next());

    use ndarray::{array, Zip};

    let k = sequence_gen::grids_to_sequence_grid(&g, &g.next());

    println!("{}", k[[0, 0, 0]]);
    println!("{}", k[[1, 0, 0]]);
    println!("{}", k[[2, 0, 0]]);
    println!("{}", k[[3, 0, 0]]);
    println!("{}", k[[4, 0, 0]]);

    for grid in sequence_gen::seq_to_grids(&g, &g.next()) {
        println!("{}", grid);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
