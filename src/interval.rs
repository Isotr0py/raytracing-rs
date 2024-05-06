pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

#[test]
fn test_interval() {
    let i = Interval::new(0., 1.);
    assert_eq!(i.size(), 1.);
    assert_eq!(i.max(), 1.);
    assert_eq!(i.min(), 0.);
    assert!(i.contains(0.5));
    assert!(!i.contains(1.5));
    assert!(i.surrounds(0.5));
    assert!(!i.surrounds(1.));
}
