#![warn(clippy::all, missing_debug_implementations, missing_docs)]
//! A rust raytracer! Modelled after Jamis Buck's great book "The raytracer challenge".
//! It should be reasonably performant, but certainly not the fastest thing out there.
//!
//! # Optimizations
//!
//! # Features
//!
//! ## rayon
//! You can activate the "rayon" feature to enable cpu-paralellism.
//! It will utilize all cores and split the workload at rendering each row seperately.
//! ## shininess_as_float
//! Per standard, the shininess value of a material is stored as an unsized integer to improve performance, as raising a float to the power of an int is significantly faster than to the power of a float

/// A camera, used to render the world from a certain view.
pub mod camera;
/// A canvas to render the world to.
pub mod canvas;

/// The color of a point or a pixel on a canvas
pub mod color;
mod epsilon;
/// An intersection occurs when a ray hits an object
mod intersection;
/// A light source in the scene
pub mod light;
/// Every object in the scene has a material
pub mod material;
/// The nxn matrices used for computations
pub mod matrix;
pub mod pattern;
/// PPM file format logic
pub mod ppm;
/// What gives a raytracer it's name
pub mod ray;
/// All shapes reside here
pub mod shapes;
/// Vectors and Points in 3d euclidean space
pub mod tuple;
pub mod world;
