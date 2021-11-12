#![warn(clippy::all)]
/// A rust raytracer! Modelled after Jamis Buck's great book "The raytracer challenge".
/// It should be reasonably performant, but certainly not the fastest thing out there.
///
/// # Optimizations
///
/// # Features
///
/// ## rayon
/// You can activate the "rayon" feature to enable cpu-paralellism.
/// It will utilize all cores and split the workload at rendering each row seperately.
/// ## shininess_as_float
/// Per standard, the shininess value of a material is stored as an unsized integer to improve performance, as raising a float to the power of an int is significantly faster than
pub mod camera;
pub mod canvas;
pub mod color;
mod epsilon;
mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ppm;
pub mod ray;
pub mod shapes;
pub mod tuple;
pub mod world;
