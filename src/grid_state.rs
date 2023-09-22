use crate::grid_implementation::Grid;
use crate::grid_implementation::States;
use crate::grid_implementation::States::Alive;
use crate::grid_implementation::States::Dead;
use crate::grid_implementation::SIZE;

use std::fmt;

impl Grid<States> {
    fn toroidal(&self, (x, y): (i8, i8)) -> States {
        let sz = SIZE as i8;
        let (xmod, ymod) = (x.rem_euclid(sz) as usize, y.rem_euclid(sz) as usize);

        self[(xmod, ymod)]
    }

    fn rand_array(_: usize) -> [States; SIZE] {
        rand::random()
    }

    fn rand_matrix() -> [[States; SIZE]; SIZE] {
        core::array::from_fn(Self::rand_array)
    }

    pub fn new() -> Grid<States> {
        Grid {
            matrix: Self::rand_matrix(),
        }
    }

    fn get_neighbor_count(&self, (x, y): (i8, i8)) -> u8 {
        let nghbhd: [(i8, i8); 9] = [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y),
        ];

        nghbhd
            .into_iter()
            .fold(0, |acc, b| acc + (self.toroidal(b) as u8))
    }

    pub fn next(&self) -> Grid<States> {
        let mut next_gen = self.clone();

        for i in 0..SIZE {
            for j in 0..SIZE {
                let nghbr_count: u8 = next_gen.get_neighbor_count((i as i8, j as i8));

                match nghbr_count {
                    3 => next_gen.matrix[i][j] = Alive,
                    4 => continue,
                    _ => next_gen.matrix[i][j] = Dead,
                }
            }
        }
        next_gen
    }

    fn state_to_emote(state: States) -> String {
        match state {
            Alive => "🌕".to_string(),
            Dead => "🌑".to_string(),
        }
    }

    fn array_to_emotes(line: [States; SIZE]) -> String {
        line.into_iter().fold(String::new(), |acc, state| {
            acc + &Self::state_to_emote(state)
        })
    }
}

impl fmt::Display for Grid<States> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.matrix.into_iter().fold(String::new(), |acc, line| {
                acc + &Grid::array_to_emotes(line) + "\n"
            })
        )
    }
}
