use raytracerchallenge::{
    matrix::{Mat4, IDENTITY_MATRIX_4},
    tuple::Point,
};
#[mutants::skip]
fn main() {
    invert_identity();
    multiply_by_inverse();
    inverse_transpose();
    identity_tuple();
}
#[mutants::skip]
fn invert_identity() {
    let m = IDENTITY_MATRIX_4;
    println!("The inverted identity matrix: \n\n{:?}\n\n", m.inverse());
}
#[mutants::skip]
fn multiply_by_inverse() {
    let a = Mat4::new([
        [3., -9., 7., 3.],
        [3., -8., 2., -9.],
        [-4., 4., 4., 1.],
        [-6., 5., -1., 1.],
    ]);

    println!(
        "A matrix multiplied by its inverse looks like this: \n\n{:?}\n\n",
        a * a.inverse()
    );
}
#[mutants::skip]
fn inverse_transpose() {
    let a = Mat4::new([
        [3., -9., 7., 3.],
        [3., -8., 2., -9.],
        [-4., 4., 4., 1.],
        [-6., 5., -1., 1.],
    ]);

    println!(
        "The inverse of the transpose:\n\n{:?}\n\nThe other way around:\n\n{:?}\n\n",
        a.inverse().transpose(),
        a.transpose().inverse()
    );
}
#[mutants::skip]
fn identity_tuple() {
    let mut a = IDENTITY_MATRIX_4;
    let mut b = IDENTITY_MATRIX_4;
    let mut c = IDENTITY_MATRIX_4;
    let mut d = IDENTITY_MATRIX_4;

    let tuple = Point::new(2., 4., 6.);

    a[0][0] = 3.;
    b[1][1] = 3.;
    c[2][2] = 3.;
    d[3][3] = 3.;

    println!(
        "Multiplied identity (each has one of the 1s changed to 3) by tuple: \n\n{:?}\n\n{:?}\n\n{:?}\n\n",
        a * tuple,
        b * tuple,
        c * tuple,
    );
}
