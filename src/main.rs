use image::RgbImage;
use std::path::Path;

use oakland_raders::geometry::{Pt, Sphere};
use oakland_raders::scene::Scene;

fn main() {
    let mut scene = Scene::new();
    scene.add_obj(Sphere::new(Pt::new(0.0, 0.0, 500.0), 100.0));
    let capture = scene.capture(500, 300);

    let mut img = RgbImage::new(500, 300);

    for x in 0..500 {
        for y in 0..300 {
            img.put_pixel(x, y, capture.get_pixel((x.try_into().unwrap(), y.try_into().unwrap())));
        }
    }

    img.save(Path::new("image.png")).unwrap();
}
