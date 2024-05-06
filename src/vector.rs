use std::ops::{Add, Div, Mul, Sub};

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

impl Vec3 {
    pub fn length_squared(self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
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
}

#[test]
fn test_gen() {
    let vec1: Vec3 = Vec3::new([0.2, 0.4, 0.8]);
    let vec2: Vec3 = Vec3::from_xyz(0.2, 0.4, 0.8);
    assert_eq!(vec1, vec2)
}
