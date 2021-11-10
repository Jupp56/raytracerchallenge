use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::epsilon::epsilon_equal;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub const fn const_new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn reflect(&self, p: Vector) -> Vector {
        *self - p * 2.0 * self.dot(p)
    }
}

impl Point {
    pub fn new<R: Into<f64>, S: Into<f64>, T: Into<f64>>(x: R, y: S, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn const_new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        epsilon_equal(self.x, other.x)
            && epsilon_equal(self.y, other.y)
            && epsilon_equal(self.z, other.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        epsilon_equal(self.x, other.x)
            && epsilon_equal(self.y, other.y)
            && epsilon_equal(self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tuple_tests {
    use crate::tuple::{Point, Vector};
    #[test]
    fn test_new_point() {
        let a = Point::new(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
    }

    #[test]
    fn test_const_new_point() {
        let a = Point::const_new(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
    }

    #[test]
    fn test_new_vector() {
        let a = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
    }

    #[test]
    fn test_const_new_vector() {
        let a = Vector::const_new(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
    }

    #[test]
    fn test_equality() {
        let a = Vector::new(4.3, -4.2, 3.1);
        let b = Vector::new(4.3, -4.2, 3.1);
        assert!(a == b);

        let a = Vector::new(4.3, -4.2, 3.1);
        let b = Vector::new(4.4, -4.2, 3.1);
        assert!(a != b);

        let a = Vector::new(4.3, -4.2, 3.1);
        let b = Vector::new(4.3, -4.3, 3.1);
        assert!(a != b);

        let a = Vector::new(4.3, -4.2, 3.1);
        let b = Vector::new(4.3, -4.2, 3.0);
        assert!(a != b);
    }

    #[test]
    fn test_addition() {
        let a = Point::new(3.0, -2.0, 5.0);
        let b = Vector::new(-2.0, 3.0, 1.0);
        let reference = Point::new(1.0, 1.0, 6.0);
        assert_eq!(a + b, reference);
    }

    #[test]
    fn test_subtract_two_points() {
        let a = Point::new(3.0, 2.0, 1.0);
        let b = Point::new(5.0, 6.0, 7.0);
        let reference = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(a - b, reference);
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let a = Point::new(3.0, 2.0, 1.0);
        let b = Vector::new(5.0, 6.0, 7.0);
        let reference = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(a - b, reference);
    }

    #[test]
    fn test_subtract_vector_from_vector() {
        let a = Vector::new(3.0, 2.0, 1.0);
        let b = Vector::new(5.0, 6.0, 7.0);
        let reference = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(a - b, reference);
    }

    #[test]
    fn negate_vector() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let reference = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(-a, reference);
    }

    #[test]
    fn multiply_by_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let reference = Vector::new(3.5, -7.0, 10.5);
        assert_eq!(a * 3.5, reference);
    }

    #[test]
    fn multiply_by_fraction() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let reference = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(a * 0.5, reference);
    }

    #[test]
    fn divide_by_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let reference = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(a / 2.0, reference);
    }

    #[test]
    fn magnitude() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0f64.sqrt());
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0f64.sqrt());
    }

    #[test]
    fn normalized() {
        let v = Vector::new(4., 0., 0.0);
        let reference = Vector::new(1., 0., 0.);
        assert_eq!(v.normalized(), reference);

        let v = Vector::new(1., 2., 3.0);
        let reference = Vector::new(0.26726, 0.53452, 0.80178);
        assert_eq!(v.normalized(), reference);
    }

    #[test]
    fn normalize() {
        let mut v = Vector::new(4., 0., 0.0);
        let reference = Vector::new(1., 0., 0.);
        v.normalize();
        assert_eq!(v, reference);

        let mut v = Vector::new(1., 2., 3.0);
        let reference = Vector::new(0.26726, 0.53452, 0.80178);
        v.normalize();
        assert_eq!(v, reference);
    }

    #[test]
    fn dot_product() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert_eq!(a.dot(b), 20.);
    }

    #[test]
    fn cross() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        let cross_a_b = Vector::new(-1., 2., -1.);
        let cross_b_a = Vector::new(1., -2., 1.);
        assert_eq!(a.cross(b), cross_a_b);
        assert_eq!(b.cross(a), cross_b_a);
    }

    #[test]
    fn reflect_45() {
        let v = Vector::new(1, -1, 0);
        let n = Vector::new(0, 1, 0);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1, 1, 0));
    }

    #[test]
    fn reflect_slanted() {
        let v = Vector::new(0, -1, 0);
        let n = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1, 0, 0));
    }
}
