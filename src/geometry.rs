use std::fmt;
use std::ops;
use image::Rgb;

#[derive(Debug, PartialEq)]
pub struct Vec3(f64, f64, f64);

pub trait Renderable: fmt::Debug {
    fn intersects(&self, r: &Ray) -> bool;
    fn intersects_at(&self, r: &Ray) -> Vec<Vec3>;
    fn get_luminosity(&self) -> u8;
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn zero() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn infinity() -> Self {
        Vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY)
    }

    pub fn scaled(&self, scale: f64) -> Self {
        Vec3(self.0 * scale, self.1 * scale, self.2 * scale)
    }

    pub fn mag(&self) -> f64 {
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub fn unit(&self) -> Self {
        let mag = self.mag();
        Vec3(self.0/mag, self.1/mag, self.2/mag)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3(
            self.0+other.0,
            self.1+other.1,
            self.2+other.2,
        )
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3(
            self.0-other.0,
            self.1-other.1,
            self.2-other.2,
        )
    }
}

#[derive(Debug)]
pub struct Ray {
    o: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Self {
        Ray { o, dir }
    }

    pub fn from_origin(dir: Vec3) -> Self {
        Ray { o: Vec3::zero(), dir }
    }

    pub fn reflected(&self, axis: &Ray) -> Self {
        let projected = axis.dir.scaled(self.dir.dot(&axis.dir));
        Ray {
            o: &axis.o + &axis.dir,
            dir: &self.dir - &projected.scaled(2.0),
        }
    }
}

#[derive(Debug)]
pub struct Sphere {
    c: Vec3,           // Center
    r: f64,          // Radius
    color: Rgb<u8>,  // Color
    lum: u8,         // Luminosity
    spec: u8,        // Specular reflection
    diff: u8,        // Diffuse reflection
}

impl Sphere {
    pub fn new(c: Vec3, r: f64) -> Self {
        Sphere { c, r, color: Rgb([255, 255, 255]), lum: 10, spec: 255, diff: 255 }
    }

    pub fn set_color(&mut self, color: Rgb<u8>) {
        self.color = color;
    }

    pub fn set_lum(&mut self, lum: u8) {
        self.lum = lum;
    }

    pub fn set_spec(&mut self, spec: u8) {
        self.spec = spec;
    }

    pub fn set_diff(&mut self, diff: u8) {
        self.diff = diff;
    }
}

impl Renderable for Sphere {
    fn intersects(&self, r: &Ray) -> bool {
        //                            (PxCx + PyCy + PzCz) ^2
        // R^2 >= Cx^2 + Cy^2 + Cz^2 - -----------------------
        //                              Px^2 + Py^2 + Pz^2
        let denom = r.dir.0 * r.dir.0 + r.dir.1 * r.dir.1 + r.dir.2 * r.dir.2;
        self.r * self.r * denom
            >= (self.c.0 * self.c.0 + self.c.1 * self.c.1 + self.c.2 * self.c.2) * denom
                - (self.c.0 * r.dir.0 + self.c.1 * r.dir.1 + self.c.2 * r.dir.2).powi(2)
    }

    fn intersects_at(&self, r: &Ray) -> Vec<Vec3> {
        let sum_of_squares = r.dir.0*r.dir.0 + r.dir.1*r.dir.1 + r.dir.2*r.dir.2;
        let discriminant = (r.dir.0*self.c.0 + r.dir.1*self.c.1 + r.dir.2*self.c.2).powi(2) -
            sum_of_squares *
            (self.c.0*self.c.0 + self.c.1*self.c.1 + self.c.2*self.c.2 -
             self.r*self.r);
        if discriminant < 0.0 {
            return vec![];
        }
        if discriminant == 0.0 {
            let scale = (r.dir.0 + r.dir.1 + r.dir.2) / sum_of_squares;
            return vec![r.dir.scaled(scale)];
        }
        let sqrt_discriminant = discriminant.sqrt();
        let scale1 = (r.dir.0*self.c.0 + r.dir.1*self.c.1 + r.dir.2*self.c.2 - sqrt_discriminant) / sum_of_squares;
        let scale2 = (r.dir.0*self.c.0 + r.dir.1*self.c.1 + r.dir.2*self.c.2 + sqrt_discriminant) / sum_of_squares;
        vec![r.dir.scaled(scale1), r.dir.scaled(scale2)]
    }

    fn get_luminosity(&self) -> u8 {
        self.lum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude() {
        let v = Vec3(3.0, 4.0, -12.0);
        assert_eq!(v.mag(), 13.0);
    }

    #[test]
    fn unit_vector() {
        let v = Vec3(3.0, -4.0, 0.0);
        assert_eq!(v.unit(), Vec3(0.6, -0.8, 0.0));
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3(1.0, 3.0, -5.0);
        let v2 = Vec3(4.0, -2.0, -1.0);
        assert_eq!(v1.dot(&v2), 3.0);
    }

    #[test]
    fn ray_hits_sphere() {
        let sphere = Sphere::new(Vec3(3.0, 4.0, 8.0), 1.0);
        let ray = Ray::from_origin(Vec3(0.5, 0.5, 1.0));
        assert!(sphere.intersects(&ray));
        assert_eq!(sphere.intersects_at(&ray).len(), 2);
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere::new(Vec3(3.0, 4.0, 8.0), 1.0);
        let ray = Ray::from_origin(Vec3(1.0, 0.5, 1.0));
        assert!(!sphere.intersects(&ray));
        assert_eq!(sphere.intersects_at(&ray).len(), 0);
    }
}
