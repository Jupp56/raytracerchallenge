use std::{ops::{Index, IndexMut, Mul, MulAssign}, collections::HashSet};

use crate::{
    epsilon::EpsilonEqual,
    tuple::{Point, Vector},
};

/// A 2x2 matrix
pub type Mat4 = Matrix<4>;
/// A 3x3 matrix
pub type Mat3 = Matrix<3>;
/// A 4x4 matrix
pub type Mat2 = Matrix<2>;

/// The 4x4 identity matrix
pub const IDENTITY_MATRIX_4: Mat4 = Matrix::new([
    [1., 0., 0., 0.],
    [0., 1., 0., 0.],
    [0., 0., 1., 0.],
    [0., 0., 0., 1.],
]);

#[derive(Copy, Clone, Debug)]
/// Matrix type, shorthand versions for dimensions 2-4 available as type [`Mat2`], [`Mat3`] and [`Mat4`].
pub struct Matrix<const SIZE: usize> {
    content: [[f64; SIZE]; SIZE],
}

impl<const SIZE: usize> Matrix<{ SIZE }> {
    /// Creates a new, empty (all values 0) matrix.
    pub const fn new_empty() -> Self {
        Matrix {
            content: [[0.; SIZE]; SIZE],
        }
    }

    /// Creates a new matrix from the given rectangular array
    pub const fn new(arr: [[f64; SIZE]; SIZE]) -> Self {
        Matrix { content: arr }
    }

    /// returns the inner array.
    pub const fn get(&self, x: usize, y: usize) -> f64 {
        self.content[x][y]
    }

    /// transposes a matrix.
    pub fn transpose(&self) -> Self {
        let mut m = Matrix::<SIZE>::new_empty();

        for x in 0..SIZE {
            for y in 0..SIZE {
                m[x][y] = self[y][x];
            }
        }

        m
    }
}

impl<const SIZE: usize> Index<usize> for Matrix<SIZE> {
    type Output = [f64; SIZE];

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl<const SIZE: usize> IndexMut<usize> for Matrix<SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
    }
}

impl<const SIZE: usize> PartialEq for Matrix<SIZE> {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..SIZE {
            for y in 0..SIZE {
                if !self.content[x][y].e_equals(other.content[x][y]) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const SIZE: usize> Mul for Matrix<SIZE> {
    type Output = Matrix<SIZE>;

    fn mul(self, rhs: Matrix<SIZE>) -> Self::Output {
        let mut res = Matrix::<SIZE>::new_empty();

        for row in 0..SIZE {
            for col in 0..SIZE {
                let mut val: f64 = 0.;
                for x in 0..SIZE {
                    val += self[row][x] * rhs[x][col];
                }
                res[row][col] = val;
            }
        }

        res
    }
}

impl<const SIZE: usize> MulAssign for Matrix<SIZE> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Point> for Mat4 {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3],
            y: self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3],
            z: self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3],
        }
    }
}

impl Mul<Vector> for Mat4 {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
            y: self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
            z: self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z,
        }
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        IDENTITY_MATRIX_4
    }
}

impl Mat2 {
    /// The determinant of this matrix
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    /// checks wether this matrix is invertible by calculating its determinant
    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}

impl Mat3 {
    /// A 2x2 submatrix of this 3x3 matrix, without the xth row and yth column
    /// # Example
    /// ```
    /// use raytracerchallenge::matrix::{Mat2, Mat3};
    /// let m3 = Mat3::new([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);
    /// let m2 = Mat2::new([[-3., 2.], [0., 6.]]);
    /// assert_eq!(m3.submatrix(0, 2), m2);
    /// ```
    pub fn submatrix(&self, i: usize, j: usize) -> Mat2 {
        let mut m = Mat2::new_empty();
        let mut passed_x = false;

        for row in 0..3 {
            if row == i {
                passed_x = true;
                continue;
            }
            let actual_x = if passed_x { row - 1 } else { row };
            let mut passed_y = false;
            for col in 0..3 {
                if col == j {
                    passed_y = true;
                    continue;
                }

                let actual_y = if passed_y { col - 1 } else { col };

                m[actual_x][actual_y] = self[row][col];
            }
        }

        m
    }

    /// Calculates the minor of this matrix by calculating the determinant of its i-j-submatrix.
    pub fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    /// Calculates the i-j-cofactor of this matrix
    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        let mut cofactor = self.minor(i, j);
        if (i + j) % 2 == 1 {
            cofactor *= -1.;
        }

        cofactor
    }

    /// Calculates the determinant of this matrix.
    pub fn determinant(&self) -> f64 {
        let mut det: f64 = 0.;

        for column in 0..3 {
            det += self[0][column] * self.cofactor(0, column);
        }

        det
    }

    /// Calculates if this matrix is invertible using its determinant
    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}

impl Mat4 {
    /// Calculates the x-y-submatrix of this matrix, which is this matrix without the xth row and yth column.
    /// # Example
    /// ```
    /// use raytracerchallenge::matrix::{Mat3, Mat4};
    /// let m4 = Mat4::new([
    /// [-6., 1., 1., 6.],
    /// [-8., 5., 8., 6.],
    /// [-1., 0., 8., 2.],
    /// [-7., 1., -1., 1.],
    /// ]);
    ///
    /// let m3 = Mat3::new([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);
    ///
    /// assert_eq!(m4.submatrix(2, 1), m3);
    /// ```
    pub fn submatrix(&self, x: usize, y: usize) -> Mat3 {
        let mut m = Mat3::new_empty();
        let mut passed_x = false;

        for row in 0..4 {
            if row == x {
                passed_x = true;
                continue;
            }
            let actual_x = if passed_x { row - 1 } else { row };
            let mut passed_y = false;
            for col in 0..4 {
                if col == y {
                    passed_y = true;
                    continue;
                }

                let actual_y = if passed_y { col - 1 } else { col };

                m[actual_x][actual_y] = self[row][col];
            }
        }

        m
    }

    /// i-j-minor of this matrix
    pub fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    /// i-j-cofactor of this matrix
    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        let mut cofactor = self.minor(i, j);
        if (i + j) % 2 == 1 {
            cofactor *= -1.;
        }

        cofactor
    }

    /// Determinant of this matrix
    pub fn determinant(&self) -> f64 {
        let mut det: f64 = 0.;
        for column in 0..4 {
            det += self[0][column] * self.cofactor(0, column);
        }
        det
    }

    /// If this matrix is invertible
    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    /// Inverts this matrix
    pub fn inverse(&self) -> Self {
        let mut m1 = Mat4::new_empty();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                m1[col][row] = c / determinant;
            }
        }

        m1
    }

    /// Creates a new 4x4-Matrix translated by x, y and z.
    pub fn new_translation<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Mat4::new([
            [1., 0., 0., x.into()],
            [0., 1., 0., y.into()],
            [0., 0., 1., z.into()],
            [0., 0., 0., 1.],
        ])
    }

    /// Translates this matrix by x, y and z.
    pub fn translate<T: Into<f64>>(&mut self, x: T, y: T, z: T) {
        *self *= Self::new_translation(x, y, z);
    }

    /// Creates a new 4x3 matrix scaled by x, y and z.
    pub fn new_scaling<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Mat4::new([
            [x.into(), 0., 0., 0.],
            [0., y.into(), 0., 0.],
            [0., 0., z.into(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// Scales this matrix by x, y and z
    pub fn scale<T: Into<f64>>(&mut self, x: T, y: T, z: T) {
        *self *= Self::new_scaling(x, y, z);
    }

    /// Creates a new rotation matrix for given x-rotation.
    pub fn new_rotation_x<T: Into<f64>>(r: T) -> Self {
        let r: f64 = r.into();
        Mat4::new([
            [1., 0., 0., 0.],
            [0., r.cos(), -r.sin(), 0.],
            [0., r.sin(), r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// rotates this matrix on the x axis
    pub fn rotate_x<T: Into<f64>>(&mut self, r: T) {
        *self *= Self::new_rotation_x(r);
    }

    /// Creates a new rotation matrix for given y-rotation.
    pub fn new_rotation_y<T: Into<f64>>(r: T) -> Self {
        let r: f64 = r.into();
        Mat4::new([
            [r.cos(), 0., r.sin(), 0.],
            [0., 1., 0., 0.],
            [-r.sin(), 0., r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// rotates this matrix on the x axis
    pub fn rotate_y<T: Into<f64>>(&mut self, r: T) {
        *self *= Self::new_rotation_y(r);
    }

    /// Creates a new rotation matrix for given z-rotation.
    pub fn new_rotation_z<T: Into<f64>>(r: T) -> Self {
        let r: f64 = r.into();
        Mat4::new([
            [r.cos(), -r.sin(), 0., 0.],
            [r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// rotates this matrix on the x axis
    pub fn rotate_z<T: Into<f64>>(&mut self, r: T) {
        *self *= Self::new_rotation_z(r);
    }

    /// new shearing matrix
    pub fn new_shearing<T: Into<f64>>(x_y: T, x_z: T, y_x: T, y_z: T, z_x: T, z_y: T) -> Self {
        Mat4::new([
            [1., x_y.into(), x_z.into(), 0.],
            [y_x.into(), 1., y_z.into(), 0.],
            [z_x.into(), z_y.into(), 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// applies shearing on this matrix
    pub fn shear<T: Into<f64>>(&mut self, x_y: T, x_z: T, y_x: T, y_z: T, z_x: T, z_y: T) {
        *self *= Self::new_shearing(x_y, x_z, y_x, y_z, z_x, z_y);
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::tuple::Point;

    use super::*;

    #[test]
    fn instantiate_4x4() {
        let m = Mat4::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.);
        assert_eq!(m[0][3], 4.);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn test_mat2() {
        let m = Mat2::new([[-3., 5.], [1., -2.]]);
        assert_eq!(m[0][0], -3.);
        assert_eq!(m[0][1], 5.);
        assert_eq!(m[1][0], 1.);
        assert_eq!(m[1][1], -2.);
    }

    #[test]
    fn test_mat3() {
        let m = Mat3::new([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);
        assert_eq!(m[0][0], -3.);
        assert_eq!(m[1][1], -2.);
        assert_eq!(m[2][2], 1.);
    }

    #[test]
    fn test_comp_equal_matrices() {
        let m1 = Mat4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let m2 = Mat4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_comp_inequal_matrices() {
        let m1 = Mat4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let m2 = Mat4::new([
            [2., 3., 4., 5.],
            [6., 7., 8., 9.],
            [8., 7., 6., 5.],
            [4., 3., 2., 1.],
        ]);
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_mul() {
        let m1 = Mat4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let m2 = Mat4::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        let reference = Mat4::new([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);

        assert_eq!(m1 * m2, reference);
    }

    #[test]
    fn mul_by_tuple() {
        let m = Mat4::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);

        let b = Point::new(1., 2., 3.);

        let reference = Point::new(18., 24., 33.);

        assert_eq!(m * b, reference);
    }

    #[test]
    fn transpose() {
        let m = Mat4::new([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        ]);

        let mt = Mat4::new([
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        ]);

        assert_eq!(m.transpose(), mt);
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(IDENTITY_MATRIX_4.transpose(), IDENTITY_MATRIX_4);
    }

    #[test]
    fn determinant_2x2() {
        let m = Mat2::new([[1., 5.], [-3., 2.]]);
        assert_eq!(m.determinant(), 17.);
    }

    #[test]
    fn submatrix_2x2_of_3x3() {
        let m3 = Mat3::new([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);

        let m2 = Mat2::new([[-3., 2.], [0., 6.]]);

        assert_eq!(m3.submatrix(0, 2), m2);
    }

    #[test]
    fn submatrix_3x3_of_4x4() {
        let m4 = Mat4::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);

        let m3 = Mat3::new([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);

        assert_eq!(m4.submatrix(2, 1), m3);
    }

    #[test]
    fn minor_3x3() {
        let m3 = Mat3::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);

        assert_eq!(m3.minor(1, 0), 25.);
    }

    #[test]
    fn cofactor_3x3() {
        let m3 = Mat3::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);

        assert_eq!(m3.cofactor(0, 0), -12.);
        assert_eq!(m3.cofactor(1, 0), -25.);
    }

    #[test]
    fn determinant_3x3() {
        let m3 = Mat3::new([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);

        assert_eq!(m3.cofactor(0, 0), 56.);
        assert_eq!(m3.cofactor(0, 1), 12.);
        assert_eq!(m3.cofactor(0, 2), -46.);
        assert_eq!(m3.determinant(), -196.);
    }

    #[test]
    fn inverse_possible_2x2() {
        let m_inv = Mat2::new([[6., 4.], [5., 5.]]);

        let m_non_inv = Mat2::new([[-4., 2.], [0., 0.]]);

        assert!(m_inv.invertible());
        assert!(!m_non_inv.invertible());
    }

    #[test]
    fn inverse_possible_3x3() {
        let m_inv = Mat3::new([[6., 4., 4.], [5., 5., 7.], [4., -9., 3.]]);

        let m_non_inv = Mat3::new([[-4., 2., -2.], [9., 6., 2.], [0., 0., 0.]]);

        assert!(m_inv.invertible());
        assert!(!m_non_inv.invertible());
    }

    #[test]
    fn determinant_4x4() {
        let m4 = Mat4::new([
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.],
        ]);

        assert_eq!(m4.cofactor(0, 0), 690.);
        assert_eq!(m4.cofactor(0, 1), 447.);
        assert_eq!(m4.cofactor(0, 2), 210.);
        assert_eq!(m4.cofactor(0, 3), 51.);
        assert_eq!(m4.determinant(), -4071.);
    }

    #[test]
    fn inverse_possible_4x4() {
        let m_inv = Mat4::new([
            [6., 4., 4., 4.],
            [5., 5., 7., 6.],
            [4., -9., 3., -7.],
            [9., 1., 7., -6.],
        ]);

        let m_non_inv = Mat4::new([
            [-4., 2., -2., -3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.],
        ]);

        assert!(m_inv.invertible());
        assert!(!m_non_inv.invertible());
    }

    #[test]
    fn inverse() {
        let m = Mat4::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);

        let reference = Mat4::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let b = m.inverse();

        assert_eq!(m.determinant(), 532.);
        assert_eq!(m.cofactor(2, 3), -160.);
        assert!(b[3][2].e_equals(-160. / 532.));
        assert_eq!(m.cofactor(3, 2), 105.);
        assert!(b[2][3].e_equals(105. / 532.));

        assert_eq!(b, reference);
    }

    #[test]
    fn inverse_2() {
        let a = Mat4::new([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);

        let inv_a = Mat4::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(a.inverse(), inv_a);
    }

    #[test]
    fn inverse_3() {
        let a = Mat4::new([
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.],
        ]);

        let inv_a = Mat4::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(a.inverse(), inv_a);
    }

    #[test]
    fn re_inverse() {
        let a = Mat4::new([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.],
        ]);

        let b = Mat4::new([
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.],
        ]);

        let c = a * b;

        assert_eq!(c * b.inverse(), a);
    }
}

#[cfg(test)]
mod translation_matrix_tests {
    use std::f64::consts::PI;

    use crate::{
        matrix::{Mat4, IDENTITY_MATRIX_4},
        tuple::{Point, Vector},
    };

    #[test]
    fn new_translation() {
        let translation_matrix = Mat4::new_translation(5.0_f64, -3., 2.);
        let p = Point::new(-3., 4., 5.);
        let reference = Point::new(2., 1., 7.);
        assert_eq!(translation_matrix * p, reference);
    }

    #[test]
    fn translate() {
        let translation_matrix = Mat4::new_translation(5, -3, 2);
        let mut mat = IDENTITY_MATRIX_4;
        mat.translate(5, -3, 2);
        assert_eq!(mat, translation_matrix);
    }

    #[test]
    fn multiply_by_inverse_translation() {
        let transform = Mat4::new_translation(5., -3., 2.);
        let inv = transform.inverse();
        let p = Point::new(-3., 4., 5.);
        let reference = Point::new(-8., 7., 3.);
        assert_eq!(inv * p, reference);
    }

    #[test]
    fn multiply_translation_with_vector() {
        let transform = Mat4::new_translation(5., -3., 2.);
        let v = Vector::new(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn scale_point() {
        let transform = Mat4::new_scaling(2., 3., 4.);
        let p = Point::new(-4., 6., 8.);
        let reference = Point::new(-8., 18., 32.);
        assert_eq!(transform * p, reference);
    }

    #[test]
    fn scale_vector() {
        let transform = Mat4::new_scaling(2., 3., 4.);
        let v = Vector::new(-4., 6., 8.);
        let reference = Vector::new(-8., 18., 32.);
        assert_eq!(transform * v, reference);
    }

    #[test]
    fn scale_negative() {
        let transform = Mat4::new_scaling(2., 3., 4.);
        let inv = transform.inverse();
        let v = Vector::new(-4., 6., 8.);
        let reference = Vector::new(-2., 2., 2.);
        assert_eq!(inv * v, reference);
    }

    #[test]
    fn scale_reflect() {
        let transform = Mat4::new_scaling(-1., 1., 1.);
        let p = Point::new(2., 3., 4.);
        let reference = Point::new(-2., 3., 4.);
        assert_eq!(transform * p, reference);
    }

    #[test]
    fn scale() {
        let scale_matrix = Mat4::new_scaling(2, 3, 4);
        let mut m = IDENTITY_MATRIX_4;
        m.scale(2, 3, 4);
        assert_eq!(m, scale_matrix);
    }

    #[test]
    fn rotation_x() {
        let sq2: f64 = 2.0_f64.sqrt();
        let p = Point::new(0., 1., 0.);

        let half_quarter = Mat4::new_rotation_x(PI / 4.);
        let full_quarter = Mat4::new_rotation_x(PI / 2.);

        let half_ref = Point::new(0.0, sq2 / 2., sq2 / 2.);
        let full_ref = Point::new(0., 0., 1.);
        assert_eq!(half_quarter * p, half_ref);
        assert_eq!(full_quarter * p, full_ref);
    }

    #[test]
    fn rotate_x() {
        let m = Mat4::new_rotation_x(PI / 4.);
        let mut i = IDENTITY_MATRIX_4;
        i.rotate_x(PI / 4.);
        assert_eq!(i, m);
    }

    #[test]
    fn rotation_inv_x() {
        let sq2: f64 = 2.0_f64.sqrt();
        let p = Point::new(0., 1., 0.);

        let half_quarter = Mat4::new_rotation_x(PI / 4.);
        let half_quarter = half_quarter.inverse();

        let half_ref = Point::new(0.0, sq2 / 2., -(sq2 / 2.));

        assert_eq!(half_quarter * p, half_ref);
    }

    #[test]
    fn rotation_y() {
        let sq2: f64 = 2.0_f64.sqrt();
        let p = Point::new(0., 0., 1.);

        let half_quarter = Mat4::new_rotation_y(PI / 4.);
        let full_quarter = Mat4::new_rotation_y(PI / 2.);

        let half_ref = Point::new(sq2 / 2., 0.0, sq2 / 2.);
        let full_ref = Point::new(1., 0., 0.);
        assert_eq!(half_quarter * p, half_ref);
        assert_eq!(full_quarter * p, full_ref);
    }

    #[test]
    fn rotate_y() {
        let m = Mat4::new_rotation_y(PI / 4.);
        let mut i = IDENTITY_MATRIX_4;
        i.rotate_y(PI / 4.);
        assert_eq!(i, m);
    }

    #[test]
    fn rotation_z() {
        let sq2: f64 = 2.0_f64.sqrt();
        let p = Point::new(0., 1., 0.);

        let half_quarter = Mat4::new_rotation_z(PI / 4.);
        let full_quarter = Mat4::new_rotation_z(PI / 2.);

        let half_ref = Point::new(-sq2 / 2., sq2 / 2., 0.);
        let full_ref = Point::new(-1., 0., 0.);
        assert_eq!(half_quarter * p, half_ref);
        assert_eq!(full_quarter * p, full_ref);
    }

    #[test]
    fn rotate_z() {
        let m = Mat4::new_rotation_z(PI / 4.);
        let mut i = IDENTITY_MATRIX_4;
        i.rotate_z(PI / 4.);
        assert_eq!(i, m);
    }

    #[test]
    fn shearing_x_to_y() {
        let transform = Mat4::new_shearing(1, 0, 0, 0, 0, 0);
        let p = Point::new(2, 3, 4);
        let reference = Point::new(5, 3, 4);
        assert_eq!(transform * p, reference);
    }

    #[test]
    fn shearing_x_to_z() {
        let transform = Mat4::new_shearing(0, 1, 0, 0, 0, 0);
        let p = Point::new(2, 3, 4);
        let reference = Point::new(6, 3, 4);
        assert_eq!(transform * p, reference);
    }
    #[test]
    fn shearing_y_to_x() {
        let transform = Mat4::new_shearing(0, 0, 1, 0, 0, 0);
        let p = Point::new(2, 3, 4);
        let reference = Point::new(2, 5, 4);
        assert_eq!(transform * p, reference);
    }
    #[test]
    fn shearing_y_to_z() {
        let transform = Mat4::new_shearing(0, 0, 0, 1, 0, 0);
        let p = Point::new(2, 3, 4);
        let reference = Point::new(2, 7, 4);
        assert_eq!(transform * p, reference);
    }
    #[test]
    fn shearing_z_to_x() {
        let transform = Mat4::new_shearing(0, 0, 0, 0, 1, 0);
        let p = Point::new(2, 3, 4);
        let reference = Point::new(2, 3, 6);
        assert_eq!(transform * p, reference);
    }
    #[test]
    fn shearing_z_to_y() {
        let transform = Mat4::new_shearing(0, 0, 0, 0, 0, 1);
        let p = Point::new(2, 3, 4);
        let reference = Point::new(2, 3, 7);
        assert_eq!(transform * p, reference);
    }

    #[test]
    fn shear() {
        let m = Mat4::new_shearing(1, 2, 3, 4, 5, 6);
        let mut i = IDENTITY_MATRIX_4;
        i.shear(1, 2, 3, 4, 5, 6);
        assert_eq!(m, i);
    }

    #[test]
    fn test_combination_sequence() {
        let p = Point::new(1, 0, 1);
        let a = Mat4::new_rotation_x(PI / 2.);
        let b = Mat4::new_scaling(5, 5, 5);
        let c = Mat4::new_translation(10, 5, 7);

        let p2_ref = Point::new(1, -1, 0);
        let p2 = a * p;
        assert_eq!(p2, p2_ref);

        let p3_ref = Point::new(5, -5, 0);
        let p3 = b * p2;
        assert_eq!(p3, p3_ref);

        let p4_ref = Point::new(15, 0, 7);
        let p4 = c * p3;
        assert_eq!(p4, p4_ref);
    }
}
