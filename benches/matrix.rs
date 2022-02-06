use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracerchallenge::{
    matrix::Mat4,
    ray::Ray,
    tuple::{Point, Vector},
};

fn matrix_inverse(m: Mat4) {
    let _m1 = m.inverse();
}

fn criterion_benchmark(c: &mut Criterion) {
    let m4 = Mat4::new([
        [3., -9., 7., 3.],
        [3., -8., 2., -9.],
        [-4., 4., 4., 1.],
        [-6., 5., -1., 1.],
    ]);

    c.bench_function("matrix_inverse", |b| {
        b.iter(|| {
            matrix_inverse(black_box(m4));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
