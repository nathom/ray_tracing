use crate::Ray;
use crate::{Point3, Vec3};

pub struct Camera {
    // aspect_ratio: f64,
    // viewport_height: f64,
    // viewport_width: f64,
    // focal_length: f64,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // static variables

    // ratio of width to height
}

impl Camera {
    pub fn new() -> Self {
        #[allow(non_upper_case_globals)]
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;
        let origin: Point3 = Point3::zero();
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left_corner: Point3 =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    // 0 <= u,v < 1

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
