pub struct Pt(f64, f64, f64);

pub trait Renderable {
    fn intersects(&self, r: &Ray) -> bool;
}

impl Pt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pt(x, y, z)
    }

    pub fn origin() -> Self {
        Pt(0.0, 0.0, 0.0)
    }
}

pub struct Sphere {
    c: Pt,
    r: f64,
}

pub struct Ray {
    o: Pt,
    p: Pt,
}

impl Ray {
    pub fn from_origin(p: Pt) -> Self {
        Ray { o: Pt::origin(), p }
    }

    pub fn hits(&self, s: &Sphere) -> bool {
        //                            (PxCx + PyCy + PzCz) ^2
        // R^2 >= Cx^2 + Cy^2 + Cz^2 - -----------------------
        //                              Px^2 + Py^2 + Pz^2
        let denom = self.p.0 * self.p.0 + self.p.1 * self.p.1 + self.p.2 * self.p.2;
        s.r * s.r * denom >=
            (s.c.0 * s.c.0 + s.c.1 * s.c.1 + s.c.2 * s.c.2) * denom -
            (s.c.0 * self.p.0 + s.c.1 * self.p.1 + s.c.2 * self.p.2).powi(2)
    }
}

impl Sphere {
    pub fn new(c: Pt, r: f64) -> Self {
        Sphere { c, r }
    }
}

impl Renderable for Sphere {
    fn intersects(&self, r: &Ray) -> bool {
        //                            (PxCx + PyCy + PzCz) ^2
        // R^2 >= Cx^2 + Cy^2 + Cz^2 - -----------------------
        //                              Px^2 + Py^2 + Pz^2
        let denom = r.p.0 * r.p.0 + r.p.1 * r.p.1 + r.p.2 * r.p.2;
        self.r * self.r * denom >=
            (self.c.0 * self.c.0 + self.c.1 * self.c.1 + self.c.2 * self.c.2) * denom -
            (self.c.0 * r.p.0 + self.c.1 * r.p.1 + self.c.2 * r.p.2).powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hits_sphere() {
        let sphere = Sphere::new(Pt(3.0, 4.0, 8.0), 1.0);
        let ray = Ray::from_origin(Pt(0.5, 0.5, 1.0));
        assert!(sphere.intersects(&ray));
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere::new(Pt(3.0, 4.0, 8.0), 1.0);
        let ray = Ray::from_origin(Pt(1.0, 0.5, 1.0));
        assert!(!sphere.intersects(&ray));
    }
}
