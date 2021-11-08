use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracerchallenge::{
    matrix::Mat4,
    ray::Ray,
    tuple::{Point, Vector},
};

fn ray_bench(ray: Ray, m: Mat4) {
    let _r = ray.transformed(m);
}

fn criterion_benchmark(c: &mut Criterion) {
    let ray = Ray::new(Point::new(3.5, 2.5, 1.5), Vector::new(1.5, 2.6, 3.7));
    c.bench_function("ray", |b| {
        b.iter(|| {
            ray_bench(
                ray,
                black_box(Mat4::new([
                    [2., 1., 4., 5.],
                    [2.1, 4., 3.5, 6.7],
                    [2.3, 5.6, 8.7, 9.7],
                    [5.6, 9.8, 4.3, 9.7],
                ])),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
