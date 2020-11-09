use std::ops::Add;

#[derive(Debug)]
pub struct Vec3 {
    e: (f64, f64, f64),
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: (e0, e1, e2) }
    }

    pub fn x(&self) -> f64 {
        self.e.0
    }

    pub fn y(&self) -> f64 {
        self.e.1
    }

    pub fn z(&self) -> f64 {
        self.e.2
    }

    pub fn r(&self) -> f64 {
        self.e.0
    }

    pub fn g(&self) -> f64 {
        self.e.1
    }

    pub fn b(&self) -> f64 {
        self.e.2
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e.0.powi(2) + self.e.1.powi(2) + self.e.2.powi(2)
    }

    pub fn to_unit(&self) -> Self {
        let k = 1.0 / self.length();
        Self::new(self.e.0 * k, self.e.1 * k, self.e.2 * k)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        abs_diff_eq!(self.e.0, other.e.0, epsilon = 1e-3f64)
            && abs_diff_eq!(self.e.1, other.e.1, epsilon = 1e-3f64)
            && abs_diff_eq!(self.e.2, other.e.2, epsilon = 1e-3f64)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_add() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(2.2, 0.7, 0.25);
        assert_eq!(v1 + v2, Vec3::new(5.2, 4.7, 5.25));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_abs_diff_eq!(v.length(), 7.071, epsilon = 1e-3f64);
    }

    #[test]
    fn test_squared_length() {
        let v = Vec3::new(0.3, 0.4, 0.5);
        assert_abs_diff_eq!(v.squared_length(), 0.5, epsilon = 1e-3f64);
    }

    #[test]
    fn test_to_unit() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v.to_unit(), Vec3::new(0.424, 0.565, 0.707));
    }
}
