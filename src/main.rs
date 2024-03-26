use image::RgbImage;
use std::path::Path;

use oakland_raders::geometry::{Pt, Sphere};
use oakland_raders::scene::Scene;

fn main() {
    let mut scene = Scene::new();
    scene.add_obj(Sphere::new(Pt::new(100.0, 200.0, 2000.0), 300.0));
    scene.add_obj({
        let mut sphere = Sphere::new(Pt::new(-50.0, -100.0, 2500.0), 200.0);
        sphere.set_lum(200);
        sphere
    });
    let capture = scene.capture(500, 300);  // TODO: Refactor dimensions
    // TODO: Bouncing

    let mut img = RgbImage::new(500, 300);

    for x in 0..500 {
        for y in 0..300 {
            // TODO: Make this a feature of a capture or a scene
            img.put_pixel(x, y, capture.get_pixel((x.try_into().unwrap(), y.try_into().unwrap())));
        }
    }

    img.save(Path::new("image.png")).unwrap();
}
