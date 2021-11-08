use crate::{shapes::shape::Shape, shapes::sphere::Sphere, tuple::Point};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReferenceObject<'a> {
    Sphere(&'a Sphere),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}

impl<'a> Shape<'a> for Object {
    fn local_intersect(
        &'a self,
        ray: crate::ray::Ray,
        intersections: &mut Vec<crate::intersection::Intersection<'a>>,
    ) {
        match self {
            Object::Sphere(s) => s.local_intersect(ray, intersections),
        }
    }

    fn material(&self) -> crate::material::Material {
        match self {
            Object::Sphere(s) => s.material(),
        }
    }

    fn transformation_matrix(&self) -> crate::matrix::Mat4 {
        match self {
            Object::Sphere(s) => s.transformation_matrix(),
        }
    }

    fn local_normal_at(&self, p: Point) -> crate::tuple::Vector {
        match self {
            Object::Sphere(s) => s.local_normal_at(p),
        }
    }

    fn intersect(
        &'a self,
        ray: &'a crate::ray::Ray,
        intersections: &mut Vec<crate::intersection::Intersection<'a>>,
    ) {
        match self {
            Object::Sphere(s) => s.intersect(ray, intersections),
        }
    }

    fn transform_ray_to_object_space(&self, ray: &crate::ray::Ray) -> crate::ray::Ray {
        match self {
            Object::Sphere(s) => s.transform_ray_to_object_space(ray),
        }
    }

    fn inverse_transformation_matrix(&self) -> crate::matrix::Mat4 {
        match self {
            Object::Sphere(s) => s.inverse_transformation_matrix(),
        }
    }

    fn inverse_of_transpose_of_transformation_matrix(&self) -> crate::matrix::Mat4 {
        match self {
            Object::Sphere(s) => s.inverse_of_transpose_of_transformation_matrix(),
        }
    }
}

impl<'a> Shape<'a> for ReferenceObject<'a> {
    fn local_intersect(
        &'a self,
        ray: crate::ray::Ray,
        intersections: &mut Vec<crate::intersection::Intersection<'a>>,
    ) {
        match self {
            ReferenceObject::Sphere(s) => s.local_intersect(ray, intersections),
        }
    }

    fn intersect(
        &'a self,
        ray: &'a crate::ray::Ray,
        intersections: &mut Vec<crate::intersection::Intersection<'a>>,
    ) {
        match self {
            ReferenceObject::Sphere(s) => s.intersect(ray, intersections),
        }
    }

    fn transform_ray_to_object_space(&self, ray: &crate::ray::Ray) -> crate::ray::Ray {
        match self {
            ReferenceObject::Sphere(s) => s.transform_ray_to_object_space(ray),
        }
    }

    fn inverse_transformation_matrix(&self) -> crate::matrix::Mat4 {
        match self {
            ReferenceObject::Sphere(s) => s.inverse_transformation_matrix(),
        }
    }

    fn inverse_of_transpose_of_transformation_matrix(&self) -> crate::matrix::Mat4 {
        match self {
            ReferenceObject::Sphere(s) => s.inverse_of_transpose_of_transformation_matrix(),
        }
    }

    fn material(&self) -> crate::material::Material {
        match self {
            ReferenceObject::Sphere(s) => s.material(),
        }
    }

    fn transformation_matrix(&self) -> crate::matrix::Mat4 {
        match self {
            ReferenceObject::Sphere(s) => s.transformation_matrix(),
        }
    }

    fn local_normal_at(&self, p: Point) -> crate::tuple::Vector {
        match self {
            ReferenceObject::Sphere(s) => s.local_normal_at(p),
        }
    }
}
