use crate::{
    matrix::Mat4,
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug)]
/// A ray in the world.
pub struct Ray {
    /// An origin [`Point`] the ray passes through
    pub origin: Point,
    /// The direction [`Vector`] of the ray
    pub direction: Vector,
}

impl Ray {
    /// Creates a new [`Ray`]
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }
    /// The position of the [`Ray`]
    pub fn position<T: Into<f64>>(&self, t: T) -> Point {
        let t: f64 = t.into();
        self.origin + self.direction * t
    }
    #[inline]
    /// Returns the ray transformed by a [`Matrix`]
    pub fn transformed(&self, m: Mat4) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
    #[inline]
    /// Applies the given transformation [`Mat4`] to this ray.
    pub fn transform(&mut self, m: Mat4) {
        self.origin = m * self.origin;
        self.direction = m * self.direction;
    }
}

#[cfg(test)]
mod ray_tests {
    use crate::{
        matrix::Mat4,
        ray::Ray,
        tuple::{Point, Vector},
    };

    #[test]
    fn create_and_query() {
        let origin = Point::new(1, 2, 3);
        let direction = Vector::new(4, 5, 6);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(Point::new(2, 3, 4), Vector::new(1, 0, 0));
        let p1 = Point::new(2, 3, 4);
        let p2 = Point::new(3, 3, 4);
        let p3 = Point::new(1, 3, 4);
        let p4 = Point::new(4.5, 3, 4);

        assert_eq!(r.position(0), p1);
        assert_eq!(r.position(1), p2);
        assert_eq!(r.position(-1), p3);
        assert_eq!(r.position(2.5), p4);
    }

    #[test]
    fn translated() {
        let r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Mat4::new_translation(3, 4, 5);
        let r2 = r.transformed(m);
        assert_eq!(r2.origin, Point::new(4, 6, 8));
        assert_eq!(r2.direction, Vector::new(0, 1, 0));
    }

    #[test]
    fn scaled() {
        let r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Mat4::new_scaling(2, 3, 4);
        let r2 = r.transformed(m);
        assert_eq!(r2.origin, Point::new(2, 6, 12));
        assert_eq!(r2.direction, Vector::new(0, 3, 0));
    }

    #[test]
    fn translate() {
        let mut r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Mat4::new_translation(3, 4, 5);
        r.transform(m);
        assert_eq!(r.origin, Point::new(4, 6, 8));
        assert_eq!(r.direction, Vector::new(0, 1, 0));
    }

    #[test]
    fn scale() {
        let mut r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Mat4::new_scaling(2, 3, 4);
        r.transform(m);
        assert_eq!(r.origin, Point::new(2, 6, 12));
        assert_eq!(r.direction, Vector::new(0, 3, 0));
    }
}
