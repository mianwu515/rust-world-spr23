//! An example of generating julia fractals.
extern crate image;
extern crate num;

use std::path::Path;
use image::DynamicImage::ImageLuma8;
use std::fs::File;

fn main() {
    let img = image::open(&Path::new("src/in.png")).ok().expect("Opening image failed");
    let kernel = [-1.0f32, -1.0, -1.0,
              -1.0, 8.0, -1.0,
              -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);   
    filtered.save("test.png").unwrap();
}