use crate::hittable::{HitRecord, Hittable};
use crate::{Point3, Ray};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin().clone() - self.center.clone();
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction().clone());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sprtd = discriminant.sqrt();
        let root = (-half_b - sprtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sprtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let pos = r.at(root);
        let outward_normal = (pos.clone() - self.center.clone()) / self.radius;
        let mut rec = HitRecord::new(pos, outward_normal.clone(), root);
        rec.set_face_normal(&r, outward_normal);
        Some(rec)
    }
}
