// Edge detection of Dynamic Image
extern crate image;
extern crate num;

use std::path::Path;

fn main() {
    let img = image::open(&Path::new("in.png"))
        .ok()
        .expect("Opening image failed");
    let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    filtered.save("output.png").unwrap();
}
