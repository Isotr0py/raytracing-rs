use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vec3;

pub struct HittableList {
    spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
        }
    }

    pub fn add(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn len(&self) -> usize {
        self.spheres.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, r_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = r_t.max();

        for sphere in self.spheres.iter() {
            if sphere.hit(r, Interval::new(r_t.min(), closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.set_face_normal(r, temp_rec.normal);
            }
        }

        hit_anything
    }
}

#[test]
fn test_hittable_list() {
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Vec3::zeros(), 1.);
    let sphere2 = Sphere::new(Vec3::ones(), 1.);
    world.add(sphere1);
    world.add(sphere2);

    assert_eq!(world.len(), 2);
    world.clear();
    assert!(world.is_empty())
}
