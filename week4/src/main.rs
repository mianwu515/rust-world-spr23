extern crate lyon_geom;
extern crate lyon_svg;

use lyon_geom::math::Point;
use lyon_geom::path::{builder, Path, PathEvent};
use lyon_geom::svg::PathWriter;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut path_builder = builder::PathBuilder::new();

    for i in 0..360 {
        let angle = i as f32 * std::f32::consts::PI / 180.0;
        let radius = (angle.sin() * angle.sin() * angle.cos()).abs() * 100.0;
        let x = radius * angle.cos() + 200.0;
        let y = radius * angle.sin() + 200.0;
        path_builder.path_event(PathEvent::LineTo(Point { x, y }));
    }

    let path = path_builder.build();

    let mut file = File::create("rose.svg").unwrap();
    let mut buffer = Vec::new();
    {
        let mut writer = PathWriter::new(&mut buffer);
        writer.write_path(&path, &lyon_svg::Options::default()).unwrap();
    }
    file.write_all(&buffer).unwrap();
}
