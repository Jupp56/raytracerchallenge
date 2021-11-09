use std::any::Any;

use crate::{
    intersection::Intersection,
    material::Material,
    matrix::{Mat4, IDENTITY_MATRIX_4},
    ray::Ray,
    shapes::shape::Shape,
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    transformation_matrix: Mat4,
    inverted_transformation_matrix: Mat4,
    pub material: Material,
}

impl Sphere {
    pub fn set_transformation(&mut self, m: Mat4) {
        self.transformation_matrix = m;
        self.inverted_transformation_matrix = m.inverse();
    }
}

impl Shape for Sphere {
    fn local_intersect<'a>(&'a self, ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
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

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);

        intersections.push(i1);
        intersections.push(i2);
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transformation_matrix(&self) -> Mat4 {
        self.transformation_matrix
    }

    fn local_normal_at(&self, p: Point) -> Vector {
        let res_object_space = (p - Point::new(0, 0, 0)).normalized();
        res_object_space.normalized()
    }

    fn inverse_transformation_matrix(&self) -> Mat4 {
        self.inverted_transformation_matrix
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transformation_matrix: IDENTITY_MATRIX_4,
            inverted_transformation_matrix: IDENTITY_MATRIX_4,
            material: Default::default(),
        }
    }
}

#[cfg(test)]
mod sphere_tests {

    use crate::{
        intersection::Intersection,
        material::Material,
        matrix::IDENTITY_MATRIX_4,
        ray::Ray,
        shapes::shape::Shape,
        tuple::{Point, Vector},
    };

    use super::Sphere;

    #[test]
    fn ray_sphere_local_intersection() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let reference = vec![Intersection::new(4.0, &s), Intersection::new(6.0, &s)];
        let mut xs = Vec::new();
        let r_os = s.transform_ray_to_object_space(&r);
        s.local_intersect(&r_os, &mut xs);
        assert_eq!(xs, reference);
    }

    #[test]
    fn intersect_target() {
        let r = Ray::new(Point::new(0, 1, -5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let reference = vec![Intersection::new(5.0, &s), Intersection::new(5.0, &s)];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }
    #[test]
    fn ray_originating_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let reference = vec![Intersection::new(-1, &s), Intersection::new(1, &s)];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }

    #[test]
    fn ray_miss() {
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originating_behind() {
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let reference = vec![Intersection::new(-6, &s), Intersection::new(-4, &s)];
        let mut xs = Vec::new();
        s.intersect(&r, &mut xs);
        assert_eq!(xs, reference);
    }

    #[test]
    fn has_transform() {
        let s = Sphere::default();
        assert_eq!(s.transformation_matrix, IDENTITY_MATRIX_4);
    }

    #[test]
    fn normal_at_x() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(1, 0, 0));
        assert_eq!(n, Vector::new(1, 0, 0));
    }
    #[test]
    fn normal_at_y() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0, 1, 0));
        assert_eq!(n, Vector::new(0, 1, 0));
    }
    #[test]
    fn normal_at_z() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0, 0, 1));
        assert_eq!(n, Vector::new(0, 0, 1));
    }
    #[test]
    fn normal_at_nonaxial() {
        let c = 3_f64.sqrt() / 3.;
        let s = Sphere::default();
        let n = s.normal_at(Point::new(c, c, c));
        assert_eq!(n, Vector::new(c, c, c));
    }
    #[test]
    fn normal_at_normalized() {
        let c = 3_f64.sqrt() / 3.;
        let s = Sphere::default();
        let n = s.normal_at(Point::new(c, c, c));
        assert_eq!(n, n.normalized());
    }

    #[test]
    fn instantiate() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material.ambient, 1.0);
    }
}
