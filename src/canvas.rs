use crate::color::Color;

const BASE_COLOR: Color = Color {
    red: 0.,
    green: 0.,
    blue: 0.,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Errors a canvas my throw
pub enum CanvasError {
    /// The provided coordinates are not inside the dimensions of the canvas.
    InvalidCoordinates,
}

#[derive(Clone, Debug)]
/// The canvas this renderer draws it results on.
pub struct Canvas {
    canvas: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl Canvas {
    ///
    pub fn new(width: usize, height: usize) -> Self {
        Canvas::new_with_color(width, height, BASE_COLOR)
    }
    /// A new canvas, every pixel filled with the provided [`Color`]
    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        let mut vec = Vec::with_capacity(height);
        for _i in 0..height {
            let mut inner_vec: Vec<Color> = Vec::with_capacity(width);
            for _j in 0..width {
                inner_vec.push(color)
            }
            vec.push(inner_vec);
        }
        Canvas {
            canvas: vec,
            height,
            width,
        }
    }

    /// Returns the [`Color`] of the pixel at the provided coordinates.
    /// Returns a [`CanvasError::InvalidCoordinates`] if the provided coordinates are not inside of the canvas dimensions.
    pub fn pixel_at(&self, x: usize, y: usize) -> Result<Color, CanvasError> {
        if !self.check_coordinates(x, y) {
            return Err(CanvasError::InvalidCoordinates);
        }
        Ok(self.canvas[y][x])
    }

    /// Sets the [`Color`] of the pixel at the provided coordinates.
    /// Returns a [`CanvasError::InvalidCoordinates`] if the provided coordinates are not inside of the canvas dimensions.
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), CanvasError> {
        if !self.check_coordinates(x, y) {
            return Err(CanvasError::InvalidCoordinates);
        }
        self.canvas[y][x] = color;
        Ok(())
    }

    /// Checks if the coordinates provided are valid (inside of the canvas's dimensions)
    pub const fn check_coordinates(&self, x: usize, y: usize) -> bool {
        !(x >= self.width || y >= self.height)
    }

    #[mutants::skip]
    /// Vertical size of the canvas
    pub const fn height(&self) -> usize {
        self.height
    }

    #[mutants::skip]
    /// Horizontal size of the canvas
    pub const fn width(&self) -> usize {
        self.width
    }

    #[mutants::skip]
    /// Returns the backing [`Vec`] of this canvas.
    pub fn get_canvas(&self) -> &Vec<Vec<Color>> {
        &self.canvas
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::{
        canvas::{Canvas, CanvasError},
        color::Color,
    };

    const RED: Color = Color {
        red: 1.,
        green: 0.,
        blue: 0.,
    };

    #[test]
    fn new() {
        let canvas = Canvas::new(10, 20);
        let reference_color = Color::new(0., 0., 0.);

        for x in 0..10 {
            for y in 0..20 {
                let color_at = canvas.pixel_at(x, y);
                assert_eq!(color_at.unwrap(), reference_color);
            }
        }
    }

    #[test]
    fn new_with_color() {
        let reference_color = Color::new(0.2, 0.4, 0.6);
        let canvas = Canvas::new_with_color(10, 20, reference_color);

        for x in 0..10 {
            for y in 0..20 {
                let color_at = canvas.pixel_at(x, y);
                assert_eq!(color_at.unwrap(), reference_color);
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20);

        let write_result = canvas.write_pixel(2, 3, RED);
        assert!(write_result.is_ok());
        assert_eq!(canvas.pixel_at(2, 3).unwrap(), RED);
    }

    #[test]
    fn read_invalid_pixel() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.pixel_at(10, 0), Err(CanvasError::InvalidCoordinates));
    }

    #[test]
    fn write_invalid_pixel() {
        let mut canvas = Canvas::new(10, 20);
        assert_eq!(
            canvas.write_pixel(0, 20, RED),
            Err(CanvasError::InvalidCoordinates)
        );
    }

    #[test]
    fn check_coordinates() {
        let canvas = Canvas::new(10, 20);
        assert!(canvas.check_coordinates(0, 0));
        assert!(canvas.check_coordinates(9, 19));
        assert!(!canvas.check_coordinates(10, 0));
        assert!(!canvas.check_coordinates(0, 20));
    }

    #[test]
    fn get_dimensions() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 20);
    }
}
