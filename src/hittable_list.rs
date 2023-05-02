use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        Self { objects: vec![] }
    }

    fn clear(&mut self) {
        self.objects.clear()
    }

    fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let (got_hit, temp_rec) = object.hit(r, t_min, closest_so_far);
            if got_hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        (hit_anything, rec)
    }
}
