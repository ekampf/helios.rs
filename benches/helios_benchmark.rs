#[macro_use]
extern crate criterion;
extern crate helios;

use criterion::Criterion;
use helios::scenes;
use helios::tracer::*;
use std::time::Duration;

fn weekend_spheres_benchmark_impl() {
    let width = 400;
    let height = 300;
    let samples = 1;
    let scene = scenes::weekend_spheres::get_scene(width, height, samples);
    let mut render_context = RenderContext::new(width, height);

    render_context.render(&scene, 10, None);
}

fn weekend_spheres_benchmark(c: &mut Criterion) {
    c.bench_function("balls_benchmark_impl", |b| {
        b.iter(|| balls_benchmark_impl())
    });
    //    c.bench_function("balls_benchmark_impl", |_| balls_benchmark_impl());
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::new(15, 0));
    targets= weekend_spheres_benchmark
);

criterion_main!(benches);
