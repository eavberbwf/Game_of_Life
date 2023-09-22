mod grid_state;
mod grid_transitions;

mod grid_implementation;
use crate::grid_implementation::Grid;
use crate::grid_implementation::States;
use crate::grid_implementation::Transitions;
use crate::grid_implementation::Transitions::A1;
use crate::grid_implementation::Transitions::A3;

fn main() {
    let griddy = Grid::<States>::new();

    let trans: Grid<Transitions> = Grid {
        matrix: [
            [A3, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
            [A1, A1, A1, A1, A1, A1, A1, A1, A1, A1],
        ],
    };
    println!("{}", trans);

    println!("{}", griddy);
    println!("{}", griddy.next());
    println!("{}", griddy.len());
}
