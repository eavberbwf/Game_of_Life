mod grid_implementation;
use crate::grid_implementation::Grid;

fn main() {
    let griddy = Grid::new();

    println!("{}", griddy[(0, 0)]);
    println!("{}", griddy);
    println!("{}", griddy.next());
    println!("{}", griddy.len());
}
