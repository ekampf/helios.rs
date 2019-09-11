#[macro_use]
extern crate criterion;
extern crate rustybeam;

use criterion::Criterion;
use rustybeam::scenes;
use rustybeam::tracer::*;
use std::time::Duration;

fn balls_benchmark_impl() {
    let width = 400;
    let height = 300;
    let samples = 1;
    let scene = scenes::weekend_balls::get_scene(width, height, samples);
    let mut render_context = RenderContext::new(width, height);

    render_context.render(&scene, 10, None);
}

fn balls_benchmark(c: &mut Criterion) {
    c.bench_function("balls_benchmark_impl", |b| {
        b.iter(|| balls_benchmark_impl())
    });
    //    c.bench_function("balls_benchmark_impl", |_| balls_benchmark_impl());
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::new(15, 0));
    targets= balls_benchmark
);

criterion_main!(benches);
