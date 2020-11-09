use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
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

    pub fn dot(&self, other: Self) -> f64 {
        self.e.0 * other.e.0 + self.e.1 * other.e.1 + self.e.2 * other.e.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            e: (
                self.e.1 * other.e.2 - self.e.2 * other.e.1,
                -(self.e.0 * other.e.2 - self.e.2 * other.e.0),
                self.e.0 * other.e.1 - self.e.1 * other.e.0,
            ),
        }
    }

    pub fn to_unit(&self) -> Self {
        let k = 1.0 / self.length();
        Self::new(self.e.0 * k, self.e.1 * k, self.e.2 * k)
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e.0.powi(2) + self.e.1.powi(2) + self.e.2.powi(2)
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

    fn add(self, other: Self) -> Self::Output {
        Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            e: (
                self.e.0 / other.e.0,
                self.e.1 / other.e.1,
                self.e.2 / other.e.2,
            ),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            e: (self.e.0 / other, self.e.1 / other, self.e.2 / other),
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            e: (
                self.e.0 * other.e.0,
                self.e.1 * other.e.1,
                self.e.2 * other.e.2,
            ),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            e: (self.e.0 * other, self.e.1 * other, self.e.2 * other),
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output {
            e: (self * other.e.0, self * other.e.1, self * other.e.2),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: (-self.e.0, -self.e.1, -self.e.2),
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            e: (
                self.e.0 - other.e.0,
                self.e.1 - other.e.1,
                self.e.2 - other.e.2,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(2.2, 0.7, 0.25);
        assert_eq!(v1 + v2, Vec3::new(5.2, 4.7, 5.25));
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(2.0, 0.2, 0.2);
        assert_eq!(v1 / v2, Vec3::new(1.5, 20.0, 25.0));
    }

    #[test]
    fn test_div_with_f64() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v / 2.0, Vec3::new(1.5, 2.0, 2.5));
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.6, 2.0, 5.0);
        assert_eq!(-v, Vec3::new(-1.6, -2.0, -5.0));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(0.1, 2.0, 0.2);
        assert_eq!(v1 * v2, Vec3::new(0.3, 8.0, 1.0));
    }

    #[test]
    fn test_mul_with_f64() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v * 1.5, Vec3::new(4.5, 6.0, 7.5));

        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(1.5 * v, Vec3::new(4.5, 6.0, 7.5));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.6, 2.0, 5.0);
        let v2 = Vec3::new(2.2, 0.22, 0.25);
        assert_eq!(v1 - v2, Vec3::new(-0.6, 1.78, 4.75));
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

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, -2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, -1.0);
        assert_eq!(v1.dot(v2), -4.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(1.0, 0.0, -1.0);
        assert_eq!(v1.cross(v2), Vec3::new(-4.0, 8.0, -4.0));

        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1.cross(v2), Vec3::new(-2.0, 4.0, -2.0));
    }
}
