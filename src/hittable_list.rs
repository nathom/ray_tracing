use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

// Vector of hittable objects
struct HittableList {
    // going to try to avoid using reference counting, hopefully it will work
    // We also use dynamic dispatch with `dyn` because the list might
    // contain different types of hittables, so it needs to be resolved at runtime
    // Box means the object is allocated on the heap
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        Self { objects: vec![] }
    }

    fn clear(&mut self) {
        self.objects.clear()
    }

    // change to &dyn Hittable?
    fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    // Hit calculation for a single ray, where multiple objects exist
    // we do this by starting with a maximum upper bound for t (t_max)
    // and lowering it as we iterate through the list so we only end up
    // with the frontmost item in the hitrecord
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
