use std::path::Path;
use image::{RgbImage, Rgb};

use oakland_raders::geometry;
use oakland_raders::scene;

fn main() {
    let mut img = RgbImage::new(500, 500);
    let sphere = geometry::Sphere::new(geometry::Pt::new(0.0, 0.0, 0.0), 1.0);

    for x in 0..500 {
        for y in 0..500 {
            img.put_pixel(x, y, Rgb([0, (x/2).try_into().unwrap(), (y/2).try_into().unwrap()]));
        }
    }

    img.save(Path::new("image.png")).unwrap();
}
