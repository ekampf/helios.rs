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

fn two_spheres_perlin_benchmark_impl() {
    let width = 400;
    let height = 300;
    let samples = 5;
    let scene = scenes::two_spheres_perlin::get_scene(width, height, samples);
    let mut render_context = RenderContext::new(width, height);

    render_context.render(&scene, 10, None);
}

fn weekend_spheres_benchmark(c: &mut Criterion) {
    c.bench_function("weekend_spheres_benchmark_impl", |b| {
        b.iter(|| weekend_spheres_benchmark_impl())
    });
}

fn two_spheres_perlin_benchmark(c: &mut Criterion) {
    c.bench_function("two_spheres_perlin_benchmark", |b| {
        b.iter(|| two_spheres_perlin_benchmark_impl())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::new(15, 0));
    targets= weekend_spheres_benchmark, two_spheres_perlin_benchmark
);

criterion_main!(benches);
