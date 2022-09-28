use crate::{
    epsilon::EPSILON,
    intersection::Intersection,
    material::Material,
    matrix::{Mat4, IDENTITY_MATRIX_4},
    tuple::Vector,
};

use super::shape::{Shape, ShapeBound};

const NORMAL: Vector = Vector::const_new(0.0, 1.0, 0.0);

#[derive(Clone, Debug, PartialEq)]
/// A 2d, infinite plane. Comparatively cheap to render as it's normal is constant (in object space) and rays only intersect once.
pub struct Plane {
    transformation_matrix: Mat4,
    inverted_transformation_matrix: Mat4,
    material: Material,
}

impl ShapeBound for Plane {}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transformation_matrix: IDENTITY_MATRIX_4,
            inverted_transformation_matrix: IDENTITY_MATRIX_4,
            material: Default::default(),
        }
    }
}

impl Shape for Plane {
    fn local_intersect<'a>(
        &'a self,
        ray: &crate::ray::Ray,
        intersections: &mut Vec<crate::intersection::Intersection<'a>>,
    ) {
        if ray.direction.y.abs() < EPSILON {
            return;
        }
        let t = (-ray.origin.y) / ray.direction.y;
        intersections.push(Intersection::new(t, self))
    }

    fn material(&self) -> &crate::material::Material {
        &self.material
    }

    fn transformation_matrix(&self) -> crate::matrix::Mat4 {
        self.transformation_matrix
    }
    fn inverse_transformation_matrix(&self) -> Mat4 {
        self.inverted_transformation_matrix
    }
    #[inline]
    fn local_normal_at(&self, _p: crate::tuple::Point) -> crate::tuple::Vector {
        NORMAL
    }

    #[mutants::skip]
    fn eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    #[mutants::skip]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[mutants::skip]
    fn material_mut(&mut self) -> &mut crate::material::Material {
        &mut self.material
    }

    fn set_transformation_matrix(&mut self, matrix: Mat4) {
        self.transformation_matrix = matrix;
        self.inverted_transformation_matrix = matrix.inverse();
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    #[mutants::skip]
    fn as_shape(&self) -> &dyn Shape {
        self
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::{
        ray::Ray,
        shapes::{plane::Plane, shape::Shape},
        tuple::{Point, Vector},
    };

    #[test]
    fn normal_is_constant() {
        let p = Plane::default();
        let n1 = p.local_normal_at(Point::new(0, 0, 0));
        let n2 = p.local_normal_at(Point::new(10, 0, -10));
        let n3 = p.local_normal_at(Point::new(-5, 0, 150));
        let n_ref = Vector::new(0, 1, 0);
        assert_eq!(n1, n_ref);
        assert_eq!(n2, n_ref);
        assert_eq!(n3, n_ref);
    }

    #[test]
    fn intersect_with_parallel_ray() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, 10, 0), Vector::new(0, 0, 1));
        let mut intersections = Vec::new();
        p.local_intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let mut intersections = Vec::new();
        p.local_intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn intersect_from_above() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, 1, 0), Vector::new(0, -1, 0));
        let p_ref: &dyn Shape = &p;
        let mut intersections = Vec::new();
        p_ref.local_intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[0].object, p_ref);
    }

    #[test]
    fn intersect_from_below() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, -1, 0), Vector::new(0, 1, 0));
        let p_ref: &dyn Shape = &p;
        let mut intersections = Vec::new();
        p_ref.local_intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[0].object, p_ref);
    }
}
