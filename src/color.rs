use std::ops::{Add, Mul, Sub};

use crate::epsilon::epsilon_equal;

/// The color black
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

/// The color white
pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

#[derive(Copy, Clone, Debug)]
/// A color, represented as it's components red, green and blue.
/// The maximum value for a component is 1.0, though computations should be able to exceed that limit.
/// Therefore, no error is thrown.
pub struct Color {
    /// the red component
    pub red: f64,
    /// the green component
    pub green: f64,
    /// the blue component
    pub blue: f64,
}

impl Color {
    /// Creates a new color
    pub fn new<T: Into<f64>>(red: T, green: T, blue: T) -> Self {
        Self {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        epsilon_equal(self.red, other.red)
            && epsilon_equal(self.green, other.green)
            && epsilon_equal(self.blue, other.blue)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[cfg(test)]
mod color_tests {
    use crate::color::Color;

    #[test]
    fn instantiate() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn partial_eq() {
        let c1 = Color::new(-0.5, 0.4, 1.7);
        let c2 = Color::new(-0.5, 0.4, 1.7);
        assert!(c1 == c2);
    }

    #[test]
    fn add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let reference = Color::new(1.6, 0.7, 1.0);
        assert_eq!(c1 + c2, reference);
    }

    #[test]
    fn sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let reference = Color::new(0.2, 0.5, 0.5);
        assert_eq!(c1 - c2, reference);
    }

    #[test]
    fn mul_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let reference = Color::new(0.4, 0.6, 0.8);
        assert_eq!(c * 2., reference);
    }

    #[test]
    fn mul_self() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);
        let reference = Color::new(0.9, 0.2, 0.04);
        assert_eq!(c1 * c2, reference);
    }
}
