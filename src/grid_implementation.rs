use rand;
use std::clone::Clone;
use std::fmt;
use std::ops::Index;

const SIZE: usize = 10;

#[derive(Clone)]
pub struct Grid {
    matrix: [[bool; SIZE]; SIZE],
}

impl Grid {
    fn rand_array(_: usize) -> [bool; SIZE] {
        rand::random()
    }

    fn rand_matrix() -> [[bool; SIZE]; SIZE] {
        core::array::from_fn(Self::rand_array)
    }

    pub fn new() -> Grid {
        Grid {
            matrix: Self::rand_matrix(),
        }
    }

    fn toroidal(&self, (x, y): (i8, i8)) -> bool {
        let sz = SIZE as i8;
        let (xmod, ymod) = (x.rem_euclid(sz) as usize, y.rem_euclid(sz) as usize);

        self[(xmod, ymod)]
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

    pub fn next(&self) -> Grid {
        let mut next_gen = self.clone();

        for i in 0..SIZE {
            for j in 0..SIZE {
                let nghbr_count: u8 = next_gen.get_neighbor_count((i as i8, j as i8));

                match nghbr_count {
                    3 => next_gen.matrix[i][j] = true,
                    4 => continue,
                    _ => next_gen.matrix[i][j] = false,
                }
            }
        }
        next_gen
    }

    fn bool_to_emote(value: bool) -> String {
        if value {
            return "🌝".to_string();
        }
        "🌚".to_string()
    }

    fn array_to_emotes(line: [bool; SIZE]) -> String {
        line.into_iter().fold(String::new(), |acc, value| {
            acc + &Self::bool_to_emote(value)
        })
    }

    pub fn len(&self) -> &usize {
        &SIZE
    }
}

impl fmt::Display for Grid {
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

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.matrix[idx.0][idx.1]
    }
}
