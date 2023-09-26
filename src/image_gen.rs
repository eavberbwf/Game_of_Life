use image::codecs::gif::GifEncoder;
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use image::{DynamicImage, Frame, GenericImage, RgbaImage};

use std::fs::{read_dir, File};
use std::path::Path;

use crate::sequence_gen::grids_to_seq;
use crate::type_impl::{States, *};

const DIM: u32 = 1000;

// Module to create images and gifs of grid generations

fn resize_from_path<P: AsRef<Path>>(path: P) -> DynamicImage {
    ImageReader::open(path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .resize(DIM / (SIZE as u32), DIM / (SIZE as u32), Nearest)
}

fn path_to_rgba<P: AsRef<Path>>(path: P) -> RgbaImage {
    resize_from_path(path).as_rgba8().unwrap().clone()
}

fn grid_to_image(grid: Grid<States>) -> Frame {
    let decoded_image_dir: Vec<_> = read_dir("emoji_images")
        .unwrap()
        .map(|x| path_to_rgba(x.unwrap().path()))
        .collect();
    let mut image = DynamicImage::new_rgba16(DIM, DIM);

    for ((i, j), state) in grid.matrix.indexed_iter() {
        let _ = image.copy_from(
            &decoded_image_dir[*state as usize],
            (j as u32) * DIM / (SIZE as u32),
            (i as u32) * DIM / (SIZE as u32),
        );
    }

    Frame::new(image.into_rgba8())
}

pub fn make_grid_gif() -> () {
    let g: Grid<States> = Grid::new();
    let gens: [(Grid<States>, Grid<States>); 20] =
        core::array::from_fn(|i| (g.nth_gen(i), g.nth_gen(i).next()));
    let full_seq = gens.iter().flat_map(grids_to_seq);
    let frames: Vec<_> = full_seq.map(grid_to_image).collect();
    let file = File::create("emoji_images/daddy.gif").unwrap();
    let mut encoder = GifEncoder::new_with_speed(file, 15);
    let _ = encoder.set_repeat(image::codecs::gif::Repeat::Infinite);
    frames
        .iter()
        .for_each(|frame| encoder.encode_frame(frame.clone()).unwrap());
}
