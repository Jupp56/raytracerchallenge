use crate::{
    matrix::Mat4,
    tuple::{Point, Vector},
};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }
    pub fn position<T: Into<f64>>(&self, t: T) -> Point {
        let t: f64 = t.into();
        self.origin + self.direction * t
    }
    pub fn transform(&self, m: Mat4) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
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
    fn translate() {
        let r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Mat4::new_translation(3, 4, 5);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Point::new(4, 6, 8));
        assert_eq!(r2.direction, Vector::new(0, 1, 0));
    }

    #[test]
    fn scale() {
        let r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = Mat4::new_scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Point::new(2, 6, 12));
        assert_eq!(r2.direction, Vector::new(0, 3, 0));
    }
}
