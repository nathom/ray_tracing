use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

// Vector of hittable objects
pub struct HittableList {
    // going to try to avoid using reference counting, hopefully it will work
    // We also use dynamic dispatch with `dyn` because the list might
    // contain different types of hittables, so it needs to be resolved at runtime
    // Box means the object is allocated on the heap
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    // change to &dyn Hittable?
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}
impl Hittable for HittableList {
    // Hit calculation for a single ray, where multiple objects exist
    // we do this by starting with a maximum upper bound for t (t_max)
    // and lowering it as we iterate through the list so we only end up
    // with the frontmost item in the hitrecord
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(r) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = r.t;
                rec = Some(r);
            }
        }

        rec
    }
}
