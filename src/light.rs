use crate::{color::Color, tuple::Point};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color
}

impl PointLight{
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity
        }
    }
}

#[cfg(test)]
pub mod point_light_tests {
    use crate::{color::Color, light::PointLight, tuple::Point};

    #[test]
    fn instantiate() {
        let intensity = Color::new(1,1,1);
        let position = Point::new(0, 0, 0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }
}