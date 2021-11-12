use crate::{
    epsilon::EPSILON,
    ray::Ray,
    shapes::shape::Shape,
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

#[derive(Debug, PartialEq)]
pub struct PreparedComputations<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new<T: Into<f64>>(t: T, object: &'a dyn Shape) -> Intersection<'a> {
        Self {
            t: t.into(),
            object,
        }
    }

    pub fn prepare_computations(&self, r: &Ray) -> PreparedComputations {
        let point = r.position(self.t);
        let normal = self.object.normal_at(point);

        let eyev = -r.direction;

        let (inside, normal) = if normal.dot(eyev) < 0.0 {
            (true, -normal)
        } else {
            (false, normal)
        };

        let over_point = point + normal * EPSILON;

        PreparedComputations {
            t: self.t,
            object: self.object,
            point,
            over_point,
            eyev,
            normalv: normal,
            inside,
        }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.t.partial_cmp(&other.t) {
            Some(core::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}

/// Computes the first (from the viewpoint of the origin of a ray) hit of the ray out of several intersections.
/// Use this to determine the object a camera actually sees.
///
/// This function consumes the contents of the "intersections" vector.
/// You can, however, re-use it later, which reduces the number of vector allocations for intersections from O(n) to O(1).
pub fn hit<'a>(intersections: &mut Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
    let mut lowest_non_neg_opt: Option<Intersection> = None;

    while let Some(intersection) = intersections.pop() {
        if intersection.t < 0.0 {
            continue;
        }
        match &mut lowest_non_neg_opt {
            None => lowest_non_neg_opt = Some(intersection),
            Some(lowest_non_neg) => {
                if intersection.t < lowest_non_neg.t {
                    lowest_non_neg_opt = Some(intersection)
                }
            }
        }
    }

    lowest_non_neg_opt
}

#[cfg(test)]
mod intersection_tests {
    use crate::{
        epsilon::epsilon_equal,
        intersection::Intersection,
        shapes::{shape::Shape, sphere::Sphere},
    };

    #[test]
    fn intersection() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i = Intersection::new(3.5, so);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, so);
    }

    #[test]
    fn intersections() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(1, so);
        let i2 = Intersection::new(2, so);
        let xs = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert!(epsilon_equal(xs[0].t, 1.));
        assert!(epsilon_equal(xs[1].t, 2.));
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::{
        epsilon::EPSILON,
        intersection::hit,
        matrix::Mat4,
        ray::Ray,
        shapes::{shape::Shape, sphere::Sphere},
        tuple::{Point, Vector},
    };

    use super::Intersection;

    #[test]
    fn positive_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(1, so);
        let i2 = Intersection::new(2, so);
        let mut xs = vec![i1, i2];
        let i = hit(&mut xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn some_negative_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(-1, so);
        let i2 = Intersection::new(1, so);
        let mut xs = vec![i1, i2];
        let i = hit(&mut xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn negative_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(-2, so);
        let i2 = Intersection::new(-1, so);
        let mut xs = vec![i1, i2];
        let i = hit(&mut xs);
        assert!(i.is_none());
    }

    #[test]
    fn lowest_nonnegative() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(5, so);
        let i2 = Intersection::new(7, so);
        let i3 = Intersection::new(-3, so);
        let i4 = Intersection::new(2, so);
        let mut xs = vec![i1, i2, i3, i4];
        let i = hit(&mut xs).unwrap();
        assert_eq!(i, i4);
    }

    #[test]
    fn precompute_state() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, -1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn precompute_not_inside() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();
        let shape = &sphere as &dyn Shape;
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn precompute_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let sphere = Sphere::default();
        let shape = &sphere as &dyn Shape;
        let i = Intersection::new(1.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn precompute_over_z() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut shape = Sphere::default();
        shape.set_transformation(Mat4::new_translation(0, 0, 1));
        let i = Intersection::new(5, &shape);
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
