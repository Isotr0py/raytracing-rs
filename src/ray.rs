use crate::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin() + t * self.direction()
    }
}

#[test]
fn test_ray() {
    let origin = Vec3::from_xyz(0., 0., 0.);
    let direction = Vec3::from_xyz(1., 1., 1.);
    let r = Ray::new(origin, direction);
    assert_eq!(r.origin(), origin);
    assert_eq!(r.direction(), direction);
    assert_eq!(r.at(10.), Vec3::from_xyz(10., 10., 10.));
    assert_eq!(r.at(0.), Vec3::zeros());
}
