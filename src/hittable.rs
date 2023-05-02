use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub struct HitRecord {
    // Point in 3-space where ray intersected object
    pub p: Point3,
    // normal vector to the surface, always facing against ray
    normal: Vec3,
    /// t value for which ray intersects surface, e.g. ray.at(t) == p
    pub t: f64,
    // is the ray facing the "front" side of the object?
    // used for ensuring the normal is always against the ray
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn normal(&self) -> Vec3 {
        return self.normal;
    }

    /// Ensure normal is always facing against ray
    /// Poor design, might change later but im keeping it here
    /// bc thats how the book did it
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // acute angle between ray-normal
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            // facing wrong direction, so we invert
            -outward_normal
        };
    }
}

// A trait (interface) that a type of object can implement
pub trait Hittable {
    // Return (did the ray hit the surface, details of hit or garbage if it didnt hit)
    // change to Option<HitRecord>?
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}
