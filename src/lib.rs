pub struct Pt(f64, f64, f64);

pub struct Sphere;

pub struct Ray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hits_sphere() {
        let sphere = Sphere::new(Pt(3.0, 4.0, 8.0), 1.0);
        let ray = Ray::from_origin(Pt(0.5, 0.5, 1.0));
        assert!(ray.hits(&sphere));
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere::new(Pt(3.0, 4.0, 8.0), 1.0);
        let ray = Ray::from_origin(Pt(1.0, 0.5, 1.0));
        assert!(!ray.hits(&sphere));
    }
}
