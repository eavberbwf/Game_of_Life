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
    image_gen::make_grid_gif();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
