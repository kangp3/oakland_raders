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

        let viewport_distance = 500.0;
        for x in 0..x_dim {
            for y in 0..y_dim {
                let p = Pt::new(x as f64 - x_bound, y as f64 - y_bound, viewport_distance);
                if x == 250 && y == 150 {
                    println!("OBJS IS: {:?}", &self.objs.iter().map(|x| x.get_luminosity()).collect::<Vec<u8>>());
                    println!("P IS: {:?}", p);
                }
                let ray = Ray::from_origin(p);
                let closest = &self.objs.iter().min_by(|a, b| {
                    let a_dist = a.as_ref().intersects_at(&ray).first().unwrap_or(&Pt::infinity()).mag();
                    let b_dist = b.as_ref().intersects_at(&ray).first().unwrap_or(&Pt::infinity()).mag();
                    if x == 250 && y == 150 {
                        println!("A DIST: {a_dist}, B DIST: {b_dist}");
                    }
                    a_dist.total_cmp(&b_dist)
                });
                if let Some(obj) = closest {
                    if x == 250 && y == 150 {
                        println!("CLOSEST IS: {:?}", obj.get_luminosity());
                    }
                    let lum = obj.get_luminosity();
                    capture.set_pixel((x, y), Rgb([lum, lum, lum]));
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
    fn it_renders_luminous_sphere() {
        let mut scene = Scene::new();
        scene.add_obj({
            let mut sphere = Sphere::new(Pt::new(0.0, 0.0, 1000.0), 100.0);
            sphere.set_lum(255);
            sphere
        });

        let capture = scene.capture(500, 300);
        assert_eq!(capture.get_pixel((250, 150)), Rgb([255, 255, 255])); // Center of sphere is white
        assert_eq!(capture.get_pixel((450, 250)), Rgb([0, 0, 0])); // Edge of capture is black
    }

    #[test]
    fn it_renders_nonluminous_sphere() {
        let mut scene = Scene::new();
        scene.add_obj(Sphere::new(Pt::new(0.0, 0.0, 1000.0), 100.0));

        let capture = scene.capture(500, 300);
        assert_eq!(capture.get_pixel((250, 150)), Rgb([10, 10, 10])); // Center of sphere is gray
        assert_eq!(capture.get_pixel((450, 250)), Rgb([0, 0, 0])); // Edge of capture is black
    }

    #[test]
    fn nonluminous_occludes_luminous_sphere() {
        let mut scene = Scene::new();
        scene.add_obj({
            let mut sphere = Sphere::new(Pt::new(0.0, 0.0, 2000.0), 100.0);
            sphere.set_lum(255);
            sphere
        });
        scene.add_obj(Sphere::new(Pt::new(0.0, 0.0, 1000.0), 100.0));

        let capture = scene.capture(500, 300);
        assert_eq!(capture.get_pixel((250, 150)), Rgb([10, 10, 10])); // Center of sphere is gray
        assert_eq!(capture.get_pixel((450, 250)), Rgb([0, 0, 0])); // Edge of capture is black
    }
}
