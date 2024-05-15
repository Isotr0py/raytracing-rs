use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Material) -> Self {
        let radius = radius.max(0.0);
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, r_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut t = (h - sqrtd) / a;
        if !r_t.contains(t) {
            t = (h + sqrtd) / a;
            if !r_t.contains(t) {
                return false;
            }
        }
        rec.t = t;
        rec.p = r.at(t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
