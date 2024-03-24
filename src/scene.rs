use image::Rgb;

use crate::geometry::{Pt, Ray, Renderable, Sphere};

pub struct Scene<'a> {
    objs: Vec<Box<dyn Renderable + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Scene { objs: vec![] }
    }

    pub fn add_obj<T: Renderable + 'a>(&mut self, obj: T) {
        self.objs.push(Box::new(obj));
    }

    pub fn capture(&mut self, x_dim: usize, y_dim: usize) -> Capture {
        let mut capture = Capture::new((x_dim, y_dim));
        let x_bound = x_dim as f64 / 2.0;
        let y_bound = y_dim as f64 / 2.0;

        let viewport_distance = 100.0;
        for x in 0..x_dim {
            for y in 0..y_dim {
                let p = Pt::new(x as f64 - x_bound, y as f64 - y_bound, viewport_distance);
                let ray = Ray::from_origin(p);
                for obj in &self.objs {
                    if obj.as_ref().intersects(&ray) {
                        capture.set_pixel((x, y), Rgb([255, 255, 255]));
                    }
                }
            }
        }
        capture
    }
}

pub struct Capture {
    pixels: Vec<Vec<Rgb<u8>>>,
}

impl Capture {
    pub fn new(dims: (usize, usize)) -> Self {
        let black = Rgb([0, 0, 0]);
        Capture{ pixels: vec![vec![black; dims.0]; dims.1] }
    }
    
    pub fn set_pixel(&mut self, coords: (usize, usize), color: Rgb<u8>) {
        // TODO: Assert coords are within the dimensions of the capture
        self.pixels[coords.1][coords.0] = color;
    }

    pub fn get_pixel(&self, coords: (usize, usize)) -> Rgb<u8> {
        self.pixels[coords.1][coords.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_renders_sphere() {
        let mut scene = Scene::new();
        scene.add_obj(Sphere::new(Pt::new(0.0, 0.0, 500.0), 100.0));

        let capture = scene.capture(500, 300);
        assert_eq!(capture.get_pixel((250, 150)), Rgb([255, 255, 255])); // Center of sphere is white
        assert_eq!(capture.get_pixel((450, 250)), Rgb([0, 0, 0])); // Edge of capture is black
    }
}
