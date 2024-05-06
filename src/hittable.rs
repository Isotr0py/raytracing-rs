use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -1. * outward_normal,
        };
        // normal = front_face ? outward_normal : -outward_normal;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, r_t: Interval, rec: &mut HitRecord) -> bool;
}
