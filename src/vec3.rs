pub struct Vec3 {
    e: (f64, f64, f64),
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: (e0, e1, e2) }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_approx_eq!(v.length(), 7.071, 1e-3f64);
    }

    #[test]
    fn test_squared_length() {
        let v = Vec3::new(0.3, 0.4, 0.5);
        assert_eq!(v.squared_length(), 0.5);
    }
}
