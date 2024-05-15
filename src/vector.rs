use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e: [f64; 3]) -> Vec3 {
        Vec3 { e }
    }

    pub fn from_xyz(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Vec3 {
        Vec3 { e: [r, g, b] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::from_xyz(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.x();
        self.e[1] += other.y();
        self.e[2] += other.z();
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::from_xyz(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::from_xyz(-self.x(), -self.y(), -self.z())
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::from_xyz(
            other.x() * self.x(),
            other.y() * self.y(),
            other.z() * self.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::from_xyz(other * self.x(), other * self.y(), other * self.z())
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::from_xyz(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::from_xyz(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zeros()
    }
}

impl Vec3 {
    pub fn length_squared(self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3::from_xyz(0., 0., 0.)
    }

    pub fn ones() -> Vec3 {
        Vec3::from_xyz(1., 1., 1.)
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut vec = Vec3::zeros();
        for i in 0..3 {
            vec.e[i] = rng.gen::<f64>();
        }
        vec
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut vec = Vec3::zeros();
        for i in 0..3 {
            vec.e[i] = rng.gen_range(min..max);
        }
        vec
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        // Here we use a simpler way to prevent recursive
        let p = Vec3::random_range(-1., 1.);
        let p = match p.length_squared() < 1. {
            true => p,
            false => p.unit_vector(),
            // false => Vec3::random_in_unit_sphere(),
        };
        p
    }

    pub fn random_on_hemisphere(self) -> Vec3 {
        let on_unit_sphere = Vec3::random_in_unit_sphere().unit_vector();
        let on_unit_sphere = match on_unit_sphere.dot(self) > 0. {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        };
        on_unit_sphere
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p = Vec3::random_range(-1., 1.);
        p.e[2] = 0.;
        let p = match p.length_squared() < 1. {
            true => p,
            false => p.unit_vector(),
        };
        p
    }
}

impl Vec3 {
    pub fn dot(self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::from_xyz(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const S: f64 = 1e-8;
        self.x().abs() < S && self.y().abs() < S && self.z().abs() < S
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2. * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1. - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }
}

#[test]
fn test_gen() {
    let vec1: Vec3 = Vec3::new([0.2, 0.4, 0.8]);
    let vec2: Vec3 = Vec3::from_xyz(0.2, 0.4, 0.8);
    assert_eq!(vec1, vec2)
}
