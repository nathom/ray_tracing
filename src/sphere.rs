use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        // no solution, does not hit sphere
        if disc < 0.0 {
            return None;
        }

        let sqrtd = disc.sqrt();

        // find nearest root st t_min <= root <= t_max
        let root1 = (-half_b - sqrtd) / a; // first try
        let root2 = (-half_b + sqrtd) / a; // other root
        let root = {
            if root1 >= t_min && root1 <= t_max {
                root1
            } else {
                if root2 >= t_min && root2 <= t_max {
                    root2
                } else {
                    return None;
                }
            }
        };

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
