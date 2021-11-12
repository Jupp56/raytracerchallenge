use crate::{
    color::{Color, BLACK},
    epsilon::epsilon_equal,
    light::PointLight,
    tuple::{Point, Vector},
};

#[cfg(feature = "shininess_as_float")]
pub type Shininess = f64;

#[cfg(not(feature = "shininess_as_float"))]
pub type Shininess = i32;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: Shininess,
}

#[cfg(feature = "shininess_as_float")]
const SHININESS_DEFAULT: Shininess = 200.0;

#[cfg(not(feature = "shininess_as_float"))]
const SHININESS_DEFAULT: Shininess = 200;

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: SHININESS_DEFAULT,
        }
    }
}

#[cfg(feature = "shininess_as_float")]
impl PartialEq for Material {
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
    pub fn new(
        color: Color,
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

    pub(crate) fn lighting(
        &self,
        light: &PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;

        let lightv = (light.position - point).normalized();

        let ambient = effective_color * self.ambient;

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

#[cfg(test)]
mod material_tests {
    use crate::{color::Color, material::Material};

    #[test]
    fn instantiate() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1, 1, 1));
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
        let m = Material::new(color, ambient, diffuse, specular, shininess);
        assert_eq!(m.color, color);
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
        m2.color = Color::new(2, 2, 2);
        assert_ne!(m, m2);

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
}

#[cfg(test)]
mod lighting_tests {
    use crate::{
        color::Color,
        light::PointLight,
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
        let result = m.lighting(&light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_offset_45() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2., -(2.0_f64.sqrt() / 2.));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_light_offset_45() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_reflection_vec() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0.0, -(2.0_f64.sqrt()) / 2., -(2.0_f64.sqrt() / 2.));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_light_behind_object() {
        let m = Material::default();
        let position = Point::new(0, 0, 0);

        let eyev = Vector::new(0, 0, -1);
        let normalv = eyev.clone();
        let light = PointLight::new(Point::new(0, 0, 10), Color::new(1, 1, 1));
        let result = m.lighting(&light, position, eyev, normalv, false);
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
        let result = m.lighting(&light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
