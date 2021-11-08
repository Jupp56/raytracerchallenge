use crate::shapes::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReferenceObject<'a> {
    Sphere(&'a Sphere),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}
