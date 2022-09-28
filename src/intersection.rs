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
    pub under_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub reflectv: Vector,
    /// refraction ingoing angle
    pub n1: f64,
    /// refraction outgoing angle
    pub n2: f64,
}

impl<'a> Intersection<'a> {
    pub fn new<T: Into<f64>>(t: T, object: &'a dyn Shape) -> Intersection<'a> {
        Self {
            t: t.into(),
            object,
        }
    }

    pub fn prepare_computations(
        &'a self,
        r: &Ray,
        intersections: &Vec<Intersection>,
    ) -> PreparedComputations {
        let point = r.position(self.t);
        let normal = self.object.normal_at(point);

        let eyev = -r.direction;

        let (inside, normal) = if normal.dot(eyev) < 0.0 {
            (true, -normal)
        } else {
            (false, normal)
        };

        let over_point = point + normal * EPSILON;
        let under_point = point - normal * EPSILON;

        let reflectv = r.direction.reflect(normal);

        let (n1, n2) = self.compute_n1_n2(intersections);

        PreparedComputations {
            t: self.t,
            object: self.object,
            point,
            over_point,
            under_point,
            eyev,
            normalv: normal,
            inside,
            reflectv,
            n1,
            n2,
        }
    }

    /// Computes the ingress and egress refraction values for this intersection
    fn compute_n1_n2(&'a self, intersections: &Vec<Intersection<'a>>) -> (f64, f64) {
        let mut containers: Vec<&dyn Shape> = Vec::new();

        let mut n1 = 0.0;
        let mut n2 = 0.0;

        for intersection in intersections {
            if intersection == self {
                if let Some(last) = containers.last() {
                    n1 = last.material().refractive_index;
                } else {
                    n1 = 1.0;
                }
            }

            if containers.contains(&intersection.object) {
                containers.retain(|x| *x != intersection.object);
            } else {
                containers.push(intersection.object)
            }

            if intersection == self {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index;
                }
                break;
            }
        }
        (n1, n2)
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
pub fn consuming_hit<'a>(intersections: &mut Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
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

/// Computes the first (from the viewpoint of the origin of a ray) hit of the ray out of several intersections.
/// Use this to determine the object a camera actually sees.
///
/// This function consumes the contents of the "intersections" vector.
/// You can, however, re-use it later, which reduces the number of vector allocations for intersections from O(n) to O(1).
pub fn hit<'a>(intersections: &Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
    let mut lowest_non_neg_opt: Option<&Intersection<'a>> = None;

    for intersection in intersections {
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

    lowest_non_neg_opt.cloned()
}

#[cfg(test)]
mod intersection_tests {
    use crate::{
        epsilon::{EpsilonEqual, EPSILON},
        intersection::Intersection,
        matrix::Mat4,
        ray::Ray,
        shapes::{plane::Plane, shape::Shape, sphere::Sphere},
        tuple::{Point, Vector},
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
        assert!(xs[0].t.e_equals(1.));
        assert!(xs[1].t.e_equals(2.));
    }

    #[test]
    fn test_precompute_state() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r, &vec![i]);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, -1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn test_precompute_not_inside() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let sphere = Sphere::default();
        let shape = &sphere as &dyn Shape;
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r, &vec![i]);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn test_precompute_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let sphere = Sphere::default();
        let shape = &sphere as &dyn Shape;
        let i = Intersection::new(1.0, shape);
        let comps = i.prepare_computations(&r, &vec![i]);
        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn test_precompute_over_z() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut shape = Sphere::default();
        shape.set_transformation_matrix(Mat4::new_translation(0, 0, 1));
        let i = Intersection::new(5, &shape);
        let comps = i.prepare_computations(&r, &vec![i]);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn test_prepare_computations_reflection_vec() {
        let shape = Plane::default();
        let r = Ray::new(
            Point::new(0, 1, -1),
            Vector::const_new(0.0, -(2.0_f64.sqrt()), 2.0_f64.sqrt()),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r, &vec![i]);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2.0_f64.sqrt(), 2.0_f64.sqrt())
        );
    }

    #[test]
    fn refraction_intersections() {
        let mut a = Sphere::new_glass();
        a.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        a.material_mut().refractive_index = 1.5;

        let mut b = Sphere::new_glass();
        b.set_transformation_matrix(Mat4::new_translation(0., 0., -0.25));
        b.material_mut().refractive_index = 2.0;

        let mut c = Sphere::new_glass();
        c.set_transformation_matrix(Mat4::new_translation(0., 0., 0.25));
        c.material_mut().refractive_index = 2.5;

        let r = Ray::new(Point::new(0, 0, -4), Vector::new(0., 0., 0.25));

        let intersections = vec![
            Intersection { t: 2.0, object: &a },
            Intersection {
                t: 2.75,
                object: &b,
            },
            Intersection {
                t: 3.25,
                object: &c,
            },
            Intersection {
                t: 4.75,
                object: &b,
            },
            Intersection {
                t: 5.25,
                object: &c,
            },
            Intersection { t: 6.0, object: &a },
        ];

        param_test_n1_n2(0, &r, 1.0, 1.5, &intersections);
        param_test_n1_n2(1, &r, 1.5, 2.0, &intersections);
        param_test_n1_n2(2, &r, 2.0, 2.5, &intersections);
        param_test_n1_n2(3, &r, 2.5, 2.5, &intersections);
        param_test_n1_n2(4, &r, 2.5, 1.5, &intersections);
        param_test_n1_n2(5, &r, 1.5, 1.0, &intersections);
    }

    fn param_test_n1_n2(
        index: usize,
        r: &Ray,
        n1: f64,
        n2: f64,
        intersections: &Vec<Intersection>,
    ) {
        let comps = intersections[index].prepare_computations(r, intersections);
        assert_eq!(comps.n1, n1);
        assert_eq!(comps.n2, n2);
    }

    #[test]
    fn test_under_point() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));

        let mut shape = Sphere::new_glass();

        shape.set_transformation_matrix(Mat4::new_translation(0, 0, 1));

        let i = Intersection::new(5, &shape);

        let xs = vec![i];

        let comps = i.prepare_computations(&r, &xs);

        assert!(comps.under_point.z > EPSILON / 2.0);

        assert!(comps.point.z < comps.under_point.z);
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::{
        intersection::consuming_hit,
        shapes::{shape::Shape, sphere::Sphere},
    };

    use super::Intersection;

    #[test]
    fn positive_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(1, so);
        let i2 = Intersection::new(2, so);
        let mut xs = vec![i1, i2];
        let i = consuming_hit(&mut xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn some_negative_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(-1, so);
        let i2 = Intersection::new(1, so);
        let mut xs = vec![i1, i2];
        let i = consuming_hit(&mut xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn negative_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(-2, so);
        let i2 = Intersection::new(-1, so);
        let mut xs = vec![i1, i2];
        let i = consuming_hit(&mut xs);
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
        let i = consuming_hit(&mut xs).unwrap();
        assert_eq!(i, i4);
    }
}

#[cfg(test)]
mod non_consuming_hit_tests {
    use crate::{
        intersection::hit,
        shapes::{shape::Shape, sphere::Sphere},
    };

    use super::Intersection;

    #[test]
    fn positive_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(1, so);
        let i2 = Intersection::new(2, so);
        let xs = vec![i1, i2];
        let i = hit(&xs).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn some_negative_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(-1, so);
        let i2 = Intersection::new(1, so);
        let xs = vec![i1, i2];
        let i = hit(&xs).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    fn negative_t() {
        let s = Sphere::default();
        let so = &s as &dyn Shape;
        let i1 = Intersection::new(-2, so);
        let i2 = Intersection::new(-1, so);
        let xs = vec![i1, i2];
        let i = hit(&xs);
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
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs).unwrap();
        assert_eq!(i, i4);
    }
}
