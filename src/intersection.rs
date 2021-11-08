use crate::{epsilon::EPSILON, shapes::shape::Shape, object::ReferenceObject, ray::Ray, tuple::{Point, Vector}};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: ReferenceObject<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PreparedComputations<'a> {
    pub t: f64,
    pub object: ReferenceObject<'a>,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new<T: Into<f64>>(t: T, object: ReferenceObject<'a>) -> Self {
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

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let mut lowest_non_neg_opt = None;
    for intersection in intersections {
        if intersection.t < 0.0 {
            continue;
        }
        match lowest_non_neg_opt {
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
        epsilon::epsilon_equal, intersection::Intersection, object::ReferenceObject, shapes::sphere::Sphere,
    };

    #[test]
    fn intersection() {
        let s = Sphere::default();
        let so = ReferenceObject::Sphere(&s);
        let i = Intersection::new(3.5, so);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, so);
    }

    #[test]
    fn intersections() {
        let s = Sphere::default();
        let so = ReferenceObject::Sphere(&s);
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
        object::{self, ReferenceObject},
        ray::Ray,
        shapes::sphere::Sphere,
        tuple::{Point, Vector},
    };

    use super::Intersection;

    #[test]
    fn positive_t() {
        let s = Sphere::default();
        let so = ReferenceObject::Sphere(&s);
        let i1 = Intersection::new(1, so);
        let i2 = Intersection::new(2, so);
        let xs = vec![i1, i2];
        let i = hit(xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn some_negative_t() {
        let s = Sphere::default();
        let so = ReferenceObject::Sphere(&s);
        let i1 = Intersection::new(-1, so);
        let i2 = Intersection::new(1, so);
        let xs = vec![i1, i2];
        let i = hit(xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn negative_t() {
        let s = Sphere::default();
        let so = ReferenceObject::Sphere(&s);
        let i1 = Intersection::new(-2, so);
        let i2 = Intersection::new(-1, so);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn lowest_nonnegative() {
        let s = Sphere::default();
        let so = ReferenceObject::Sphere(&s);
        let i1 = Intersection::new(5, so);
        let i2 = Intersection::new(7, so);
        let i3 = Intersection::new(-3, so);
        let i4 = Intersection::new(2, so);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(xs).unwrap();
        assert_eq!(i, i4);
    }

    #[test]
    fn precompute_state() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, object::ReferenceObject::Sphere(&shape));
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
        let shape = ReferenceObject::Sphere(&sphere);
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn precompute_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let sphere = Sphere::default();
        let shape = ReferenceObject::Sphere(&sphere);
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
        let i = Intersection::new(5, ReferenceObject::Sphere(&shape));
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
