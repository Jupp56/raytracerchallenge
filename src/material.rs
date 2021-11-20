use crate::{
    color::{Color, BLACK},
    epsilon::epsilon_equal,
    light::PointLight,
    pattern::Pattern,
    shapes::shape::Shape,
    tuple::{Point, Vector},
};

#[cfg(feature = "shininess_as_float")]
/// The shininess of a material. This type exists to facilitate usage of the feature "shininess_as_float" (documented at the crate root).
pub type Shininess = f64;

#[cfg(not(feature = "shininess_as_float"))]
/// The shininess of a material. This type exists to facilitate usage of the feature "shininess_as_float" (documented at the crate root).
pub type Shininess = i32;

#[derive(Clone, Debug)]
/// The material any object in the rendered world must have.
/// The materials actual color at a given world position can be determined using its ```lighting()``` method which uses the phong shading model.
pub struct Material {
    /// The Color of the material
    pub color: ColorType,
    /// Ambient factor used in the color rendering
    pub ambient: f64,
    /// Diffuse factor used in the color rendering
    pub diffuse: f64,
    /// Specular factor used in the color rendering
    pub specular: f64,
    /// Shininess factor used in the color rendering.
    /// For performance reasons, this is an ```i32``` by default. Use the "shininess_as_float" feature to switch over to floating point.
    pub shininess: Shininess,
}

#[cfg(feature = "shininess_as_float")]
const SHININESS_DEFAULT: Shininess = 200.0;

#[cfg(not(feature = "shininess_as_float"))]
const SHININESS_DEFAULT: Shininess = 200;

impl Default for Material {
    fn default() -> Self {
        Self {
            color: ColorType::Color(Color::new(1, 1, 1)),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: SHININESS_DEFAULT,
        }
    }
}

#[cfg(feature = "shininess_as_float")]
impl<'a> PartialEq for Material<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && epsilon_equal(self.ambient, other.ambient)
            && epsilon_equal(self.diffuse, other.diffuse)
            && epsilon_equal(self.specular, other.specular)
            && epsilon_equal(self.shininess, other.shininess)
    }
}

#[cfg(not(feature = "shininess_as_float"))]
impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && epsilon_equal(self.ambient, other.ambient)
            && epsilon_equal(self.diffuse, other.diffuse)
            && epsilon_equal(self.specular, other.specular)
            && self.shininess == other.shininess
    }
}

impl Material {
    /// Creates a new material with the given color options.
    /// # Example
    /// This creates a green-ish matte material that isnt very shiny.
    /// ```
    /// use raytracerchallenge::color::Color;
    /// use raytracerchallenge::material::ColorType;
    /// use raytracerchallenge::material::Material;
    /// let color = Color::new(0.1, 1.0, 0.5);
    /// let ambient = 0.1;
    /// let diffuse = 0.7;
    /// let specular = 0.3;
    /// let shininess = 30;
    /// let m = Material::new(ColorType::Color(color), ambient, diffuse, specular, shininess);
    /// ```
    pub fn new(
        color: ColorType,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: Shininess,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    /// Ambient = false disables the ambient factor, so that two light sources dont double the ambient factor
    pub(crate) fn lighting(
        &self,
        light: &PointLight,
        object: &dyn Shape,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
        use_ambient: bool,
    ) -> Color {
        let color = match &self.color {
            ColorType::Color(color) => *color,
            ColorType::Pattern(pattern) => pattern.apply_pattern_world_space(object, point),
        };

        let effective_color = color * light.intensity;

        let lightv = (light.position - point).normalized();

        let ambient = if use_ambient {
            effective_color * self.ambient
        } else {
            BLACK
        };

        if in_shadow {
            return ambient;
        }

        let light_dot_normal = lightv.dot(normalv);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            // light is behind object
            (BLACK, BLACK)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            let specular = if reflect_dot_eye <= 0.0 {
                // light reflects away from eye
                BLACK
            } else {
                let factor = self.compute_specular_factor(reflect_dot_eye);
                light.intensity * self.specular * factor
            };
            (diffuse, specular)
        };

        ambient + diffuse + specular
    }

    #[cfg(not(feature = "shininess_as_float"))]
    fn compute_specular_factor(&self, reflect_dot_eye: f64) -> f64 {
        reflect_dot_eye.powi(self.shininess)
    }

    #[cfg(feature = "shininess_as_float")]
    fn compute_specular_factor(&self, reflect_dot_eye: f64) -> f64 {
        reflect_dot_eye.powf(self.shininess)
    }
}

#[derive(Clone, PartialEq)]
/// The different types of colorings for a material - plain colors, patterns,...

// Computation speed is more important than some bytes - colors are only stored once per object.
#[allow(clippy::large_enum_variant)]
pub enum ColorType {
    /// A plain color everywhere on the object
    Color(Color),
    /// A pattern like stripes, checkerboard or a gradient
    Pattern(Pattern),
}

use core::fmt::Debug;

impl Debug for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Color(arg0) => f.debug_tuple("Color").field(arg0).finish(),
            Self::Pattern(arg0) => f.debug_tuple("Pattern").field(arg0).finish(),
        }
    }
}

#[cfg(test)]
mod material_tests {

    use std::rc::Rc;

    use crate::{
        color::{Color, BLACK, WHITE},
        light::PointLight,
        material::{ColorType, Material},
        matrix::IDENTITY_MATRIX_4,
        pattern::Pattern,
        shapes::sphere::Sphere,
        tuple::{Point, Vector},
    };

    #[test]
    fn instantiate() {
        let m = Material::default();
        assert_eq!(m.color, ColorType::Color(Color::new(1, 1, 1)));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200);
    }

    #[test]
    fn instantiate_new() {
        let color = Color::new(0.3, 0.4, 0.5);
        let ambient = 0.6;
        let diffuse = 0.7;
        let specular = 0.8;
        let shininess = 1;
        let m = Material::new(
            ColorType::Color(color),
            ambient,
            diffuse,
            specular,
            shininess,
        );
        assert_eq!(m.color, ColorType::Color(color));
        assert_eq!(m.ambient, ambient);
        assert_eq!(m.diffuse, diffuse);
        assert_eq!(m.specular, specular);
        assert_eq!(m.shininess, shininess);
    }

    #[test]
    fn partial_eq() {
        let m = Material::default();
        assert_eq!(m, m);

        let mut m2 = Material::default();
        m2.color = ColorType::Color(Color::new(2, 2, 2));
        assert_ne!(m, m2);

        let mut m2_2 = Material::default();
        m2_2.color = ColorType::Pattern(Pattern::new(Rc::new(|_p| WHITE), IDENTITY_MATRIX_4));
        assert_ne!(m, m2_2);

        let mut m3 = Material::default();
        m3.ambient = 34.2;
        assert_ne!(m, m3);

        let mut m4 = Material::default();
        m4.diffuse = 34.2;
        assert_ne!(m, m4);

        let mut m5 = Material::default();
        m5.specular = 34.2;
        assert_ne!(m, m5);

        let mut m6 = Material::default();
        m6.shininess = 34;
        assert_ne!(m, m6);
    }

    #[test]
    fn pattern() {
        let mut m = Material::default();
        m.color = ColorType::Pattern(Pattern::stripe(WHITE, BLACK));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), WHITE);

        let c1 = m.lighting(
            &light,
            &Sphere::default(),
            Point::new(0.9, 0, 0),
            eyev,
            normalv,
            false,
            true,
        );
        let c2 = m.lighting(
            &light,
            &Sphere::default(),
            Point::new(1.1, 0, 0),
            eyev,
            normalv,
            false,
            true,
        );
        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }
}

#[cfg(test)]
mod lighting_tests {
    use crate::{
        color::Color,
        light::PointLight,
        shapes::sphere::Sphere,
        tuple::{Point, Vector},
    };

    use super::Material;

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0, 0, -1);
        let normalv = eyev.clone();
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(
            &light,
            &Sphere::default(),
            position,
            eyev,
            normalv,
            false,
            true,
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_offset_45() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2., -(2.0_f64.sqrt() / 2.));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(
            &light,
            &Sphere::default(),
            position,
            eyev,
            normalv,
            false,
            true,
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_light_offset_45() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(
            &light,
            &Sphere::default(),
            position,
            eyev,
            normalv,
            false,
            true,
        );
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_reflection_vec() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0.0, -(2.0_f64.sqrt()) / 2., -(2.0_f64.sqrt() / 2.));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(
            &light,
            &Sphere::default(),
            position,
            eyev,
            normalv,
            false,
            true,
        );
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_light_behind_object() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0, 0, -1);
        let normalv = eyev.clone();
        let light = PointLight::new(Point::new(0, 0, 10), Color::new(1, 1, 1));
        let result = m.lighting(
            &light,
            &Sphere::default(),
            position,
            eyev,
            normalv,
            false,
            true,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_in_shadow() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1, 1, 1));
        let in_shadow = true;
        let result = m.lighting(
            &light,
            &Sphere::default(),
            position,
            eyev,
            normalv,
            in_shadow,
            true,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
