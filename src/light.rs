use crate::{color::Color, tuple::Point};

#[derive(Copy, Clone, Debug, PartialEq)]

/// A simple, omni-directional point light.
pub struct PointLight {
    /// Position of this light in the world
    pub position: Point,
    /// The color and strength of this light. Use a more dimmed color for less intensity.
    pub intensity: Color,
}

impl PointLight {
    /// Instantiates a new PointLight with the given ```position``` and the ```intensity``` as color.
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
pub mod point_light_tests {
    use crate::{color::Color, light::PointLight, tuple::Point};

    #[test]
    fn instantiate() {
        let intensity = Color::new(1, 1, 1);
        let position = Point::new(0, 0, 0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }
}
