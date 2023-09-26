use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::clone::Clone;
use std::default::Default;
use std::fmt;
use std::marker::Copy;
use std::ops::{Index, IndexMut};

use ndarray::{prelude::*, ViewRepr};
use ndarray_rand::RandomExt;

use crate::type_impl::States::*;

pub const SIZE: usize = 10;

// Module to create basic types and their necessary methods.

// Implementing possible cell states, including transitory states.

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum States {
    Dead,
    #[default]
    Alive,
    A1,
    A2,
    A3,
    D1,
    D2,
    D3,
}

impl States {
    fn state_to_emote(state: States) -> String {
        match state {
            Alive => "🌕".to_string(),
            Dead => "🌑".to_string(),
            A1 => "🌖".to_string(),
            A2 => "🌗".to_string(),
            A3 => "🌘".to_string(),
            D1 => "🌒".to_string(),
            D2 => "🌓".to_string(),
            D3 => "🌔".to_string(),
        }
    }
}

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", States::state_to_emote(*self))
    }
}

impl Distribution<States> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> States {
        match rng.gen_range(0..=1) {
            0 => Dead,
            _ => Alive,
        }
    }
}

// Grid structure involving 2x2 matrix

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T> {
    pub matrix: Array2<T>,
}

impl<T> Grid<T> {
    pub fn dim(&self) -> (usize, usize) {
        self.matrix.dim()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.matrix[(idx.0, idx.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[(idx.0, idx.1)]
    }
}

// Traits for State grids, including getting next generation and getting nth generation later.

impl Grid<States> {
    pub fn new() -> Grid<States> {
        Grid {
            matrix: Array::random((SIZE, SIZE), Standard),
        }
    }

    fn toroidal(&self, (x, y): (i8, i8)) -> States {
        let sz = SIZE as i8;
        let (xmod, ymod) = (x.rem_euclid(sz) as usize, y.rem_euclid(sz) as usize);

        self[(xmod, ymod)]
    }

    pub fn get_neighbor_count(&self, (x, y): (i8, i8)) -> u8 {
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
                let nghbr_count: u8 = self.get_neighbor_count((i as i8, j as i8));

                match nghbr_count {
                    3 => next_gen[(i, j)] = Alive,
                    4 => continue,
                    _ => next_gen[(i, j)] = Dead,
                }
            }
        }
        next_gen
    }

    pub fn nth_gen(&self, n: usize) -> Grid<States> {
        if n == 0 {
            return self.clone();
        };
        let mut gens: Vec<Grid<States>> = vec![self.clone(); n + 1];
        for i in 1usize..n + 1 {
            gens[i] = gens[i - 1].next();
        }
        gens[n].clone()
    }

    fn row_to_emotes(line: ArrayBase<ViewRepr<&States>, Dim<[usize; 1]>>) -> String {
        line.into_iter()
            .fold(String::new(), |acc, state| acc + &(state.to_string()))
    }
}

impl fmt::Display for Grid<States> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.matrix.outer_iter().fold(String::new(), |acc, line| {
                acc + &Grid::<States>::row_to_emotes(line) + "\n"
            })
        )
    }
}

impl Default for Grid<States> {
    fn default() -> Grid<States> {
        Grid {
            matrix: Array::from_elem((SIZE, SIZE), Alive),
        }
    }
}
