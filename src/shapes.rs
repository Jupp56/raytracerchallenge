use crate::{
    intersection::{Intersect, Intersection},
    material::Material,
    matrix::{Mat4, IDENTITY_MATRIX_4},
    object::ReferenceObject,
    ray::Ray,
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    transformation_matrix: Mat4,
    inverted_transformation_matrix: Mat4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transformation_matrix: IDENTITY_MATRIX_4,
            inverted_transformation_matrix: IDENTITY_MATRIX_4.inverse(),
            material: Default::default(),
        }
    }
    pub fn transformation(&self) -> Mat4 {
        self.transformation_matrix
    }
    pub fn inverted_transformation(&self) -> Mat4 {
        self.inverted_transformation_matrix
    }
    pub fn set_transformation(&mut self, m: Mat4) {
        self.transformation_matrix = m;
        self.inverted_transformation_matrix = m.inverse();
    }
    pub fn normal_at(&self, p: Point) -> Vector {
        let p_object_space = self.inverted_transformation() * p;
        let res_object_space = (p_object_space - Point::new(0, 0, 0)).normalized();
        let res_world_space = self.inverted_transformation().transpose() * res_object_space;
        res_world_space.normalized()
    }
}

impl<'a> Intersect<'a> for Sphere {
    fn intersect(&'a self, ray: &'a Ray, intersections: &mut Vec<Intersection<'a>>) {
        let ray = ray.transform(self.inverted_transformation_matrix);
        let sphere_to_ray = ray.origin - Point::new(0, 0, 0);
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0.0 {
            return;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        let i1 = Intersection::new(t1, ReferenceObject::Sphere(&self));
        let i2 = Intersection::new(t2, ReferenceObject::Sphere(&self));

        intersections.push(i1);
        intersections.push(i2);
    }
}

#[cfg(test)]
mod sphere_tests {
    use std::f64::consts::PI;

    use crate::{
        epsilon::epsilon_equal,
        intersection::{Intersect, Intersection},
        material::Material,
        matrix::{Mat4, IDENTITY_MATRIX_4},
        object::ReferenceObject,
        ray::Ray,
        tuple::{Point, Vector},
    };

    use super::Sphere;

    #[test]
    fn ray_sphere_intersection() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let reference = vec![
            Intersection::new(4.0, ReferenceObject::Sphere(&s)),
            Intersection::new(6.0, ReferenceObject::Sphere(&s)),
        ];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }

    #[test]
    fn intersect_tanget() {
        let r = Ray::new(Point::new(0, 1, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let reference = vec![
            Intersection::new(5.0, ReferenceObject::Sphere(&s)),
            Intersection::new(5.0, ReferenceObject::Sphere(&s)),
        ];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }
    #[test]
    fn ray_originating_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let reference = vec![
            Intersection::new(-1, ReferenceObject::Sphere(&s)),
            Intersection::new(1, ReferenceObject::Sphere(&s)),
        ];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }

    #[test]
    fn ray_miss() {
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originating_behind() {
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Sphere::new();
        let reference = vec![
            Intersection::new(-6, ReferenceObject::Sphere(&s)),
            Intersection::new(-4, ReferenceObject::Sphere(&s)),
        ];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }

    #[test]
    fn has_transform() {
        let s = Sphere::new();
        assert_eq!(s.transformation_matrix, IDENTITY_MATRIX_4);
    }

    #[test]
    fn intersect_scaled() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transformation(Mat4::new_scaling(2, 2, 2));
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs.len(), 2);
        assert!(epsilon_equal(xs[0].t, 3.0));
        assert!(epsilon_equal(xs[1].t, 7.0));
    }

    #[test]
    fn intersect_translated() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transformation(Mat4::new_translation(5, 0, 0));
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_at_x() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1, 0, 0));
        assert_eq!(n, Vector::new(1, 0, 0));
    }
    #[test]
    fn normal_at_y() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0, 1, 0));
        assert_eq!(n, Vector::new(0, 1, 0));
    }
    #[test]
    fn normal_at_z() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0, 0, 1));
        assert_eq!(n, Vector::new(0, 0, 1));
    }
    #[test]
    fn normal_at_nonaxial() {
        let c = 3_f64.sqrt() / 3.;
        let s = Sphere::new();
        let n = s.normal_at(Point::new(c, c, c));
        assert_eq!(n, Vector::new(c, c, c));
    }
    #[test]
    fn normal_at_normalized() {
        let c = 3_f64.sqrt() / 3.;
        let s = Sphere::new();
        let n = s.normal_at(Point::new(c, c, c));
        assert_eq!(n, n.normalized());
    }
    #[test]
    fn normal_translated() {
        let mut s = Sphere::new();
        let m = Mat4::new_translation(0, 1, 0);
        s.set_transformation(m);
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_transformed() {
        let mut s = Sphere::new();
        let m = Mat4::new_scaling(1.0, 0.5, 1.0) * Mat4::new_rotation_z(PI / 5.0);
        s.set_transformation(m);
        let n = s.normal_at(Point::new(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -(2.0_f64.sqrt() / 2.0),
        ));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn instantiate() {
        let mut s = Sphere::new();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material.ambient, 1.0);
    }
}
