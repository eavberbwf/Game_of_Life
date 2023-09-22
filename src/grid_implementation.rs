/*
Implements Grid object as a toroidal matrix of booleans.
Is printed as grid of moons.
Relevent methods: tuple indexing, next, len.
*/

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::clone::Clone;
use std::marker::Copy;
use std::ops::Index;

pub const SIZE: usize = 10;

#[derive(Copy, Clone)]
pub enum Values {
    States(States),
    Transitions(Transitions),
}

#[derive(Copy, Clone)]
pub enum States {
    Alive = 1,
    Dead = 0,
}

impl Distribution<States> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> States {
        match rng.gen_range(0..=1) {
            0 => States::Dead,
            _ => States::Alive,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Transitions {
    AliveSteady,
    DeadSteady,
    A1,
    A2,
    A3,
    D1,
    D2,
    D3,
}

#[derive(Copy, Clone)]
pub struct Grid<Values> {
    pub matrix: [[Values; SIZE]; SIZE],
}

impl<Values> Index<(usize, usize)> for Grid<Values> {
    type Output = Values;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.matrix[idx.0][idx.1]
    }
}

impl<Values> Grid<Values> {
    pub fn len(&self) -> &usize {
        &SIZE
    }
}
