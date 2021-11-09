use crate::{epsilon::EPSILON, tuple::Vector};

use super::shape::Shape;

const NORMAL: Vector = Vector::const_new(0.0, 1.0, 0.0);

#[derive(Copy, Clone, Debug)]
pub struct Plane {}

impl Plane {}

impl Default for Plane {
    fn default() -> Self {
        Self {}
    }
}

impl Shape for Plane {
    fn local_intersect(
        &self,
        ray: &crate::ray::Ray,
        _intersections: &mut Vec<crate::intersection::Intersection>,
    ) {
        if ray.direction.y.abs() < EPSILON {
            return;
        } 
        
    }

    fn material(&self) -> crate::material::Material {
        todo!()
    }

    fn transformation_matrix(&self) -> crate::matrix::Mat4 {
        todo!()
    }
    #[inline]
    fn local_normal_at(&self, _p: crate::tuple::Point) -> crate::tuple::Vector {
        NORMAL
    }

    fn box_eq(&self, _other: &dyn std::any::Any) -> bool {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn material_mut(&mut self) -> &mut crate::material::Material {
        todo!()
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::{ray::Ray, shapes::{plane::Plane, shape::Shape}, tuple::{Point, Vector}};

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
        let mut intersections = Vec::new();
         p.local_intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        //assert_eq!(intersections[0].object, ReferenceObject::Plane(&p));
    }

    #[test]
    fn intersection_from_below() {

    }
}
