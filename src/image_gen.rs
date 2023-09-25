use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImage;
use image::RgbaImage;

use std::convert::AsRef;
use std::fs::read_dir;
use std::path::Path;

use crate::type_impl::{States, *};

const DIM: u32 = 500;

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

pub fn grid_to_image(grid: &Grid<States>) -> DynamicImage {
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

    image
}
