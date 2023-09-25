mod type_impl;
use crate::type_impl::{Grid, States, States::*, SIZE};
use image::codecs::png::PngDecoder;
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use ndarray::azip;
use ndarray::prelude::*;
mod image_gen;
use image::DynamicImage;
use image_gen::*;
mod sequence_gen;
use image::GenericImageView;
use std::convert::AsRef;
use std::fs::{read_dir, DirEntry};
use std::path::Path;

fn main() {
    let g: Grid<States> = Grid::new();
    let seq = sequence_gen::seq_to_grids(&g, &g.next());
    for grid in &seq {
        println!("{}", grid);
    }
    println!("{}", seq[1]);

    let img = grid_to_image(&seq[1]);
    img.save(Path::new("emoji_images/release.png"));

    //println!("{:?}", read_dir("emoji_images").unwrap().map(|x| path_to_rgba(x.unwrap().path())).collect::<Vec<_>>());
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
