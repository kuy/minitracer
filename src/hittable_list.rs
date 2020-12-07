use crate::hittable::{HitRecord, Hittable};
use crate::Ray;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut last_rec = None;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            if let Some(rec) = object.hit(&r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                last_rec = Some(rec);
            }
        }

        last_rec
    }
}
