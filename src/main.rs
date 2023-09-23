mod type_impl;
use crate::type_impl::{Grid, States, States::*, SIZE};
use ndarray::azip;
use ndarray::prelude::*;

mod sequence_gen;

fn main() {
    let g: Grid<States> = Grid::new();
    for grid in sequence_gen::seq_to_grids(&g, &g.next()) {
        println!("{}", grid);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
