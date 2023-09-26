mod type_impl;
use crate::type_impl::{Grid, States, States::*, SIZE};
use ndarray::azip;
use ndarray::prelude::*;
use ndarray::{arr2, concatenate, Axis};
mod image_gen;
use image_gen::*;
mod sequence_gen;
use image::codecs::gif::GifEncoder;
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use image::{DynamicImage, Frame, Frames, GenericImage, ImageOutputFormat, RgbaImage};
use std::convert::AsRef;
use std::fs::{read_dir, DirEntry, File};
use std::path::Path;

fn main() {
    let g: Grid<States> = Grid {
        matrix: arr2(&[
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead],
            [
                Dead, Dead, Alive, Alive, Alive, Dead, Dead, Dead, Dead, Dead,
            ],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
        ])
    };
    image_gen::make_random_grid_gif();
    // println!("{}", g.get_neighbor_count((3,4)));
    // println!("{}", g);
    // println!("{}", g.next());
    // println!("{}", g.next().next());
    // println!("{}", g.next().next().next());
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
