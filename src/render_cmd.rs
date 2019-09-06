extern crate rayon;

use crate::tracer::geometry::Sphere;
use crate::tracer::materials::*;
use crate::tracer::*;
use cgmath::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use rand::prelude::*;
use rayon::current_num_threads;
use std::path::PathBuf;
use std::sync::Arc;

fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    let r: f64 = rng.gen();
    r
}

fn get_camera(width: u64, height: u64) -> Box<dyn Camera> {
    let width = width as f64;
    let height = height as f64;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = vec3(0.0, 0.0, 0.0);
    let up = vec3(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let camera = SimpleCamera::new(
        look_from,
        look_at,
        up,
        20.0,
        width / height,
        aperture,
        focus_dist,
    );

    Box::new(camera)
}

fn get_scene(width: u64, height: u64) -> Scene {
    let camera = get_camera(width, height);
    let mut scene = Scene::new(camera);

    let bg_sphere = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))),
    };
    scene.push(Box::new(bg_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let random_mat: f64 = rand();
            let center = Point3::new(a + 0.9 * rand(), 0.2, b + 0.9 * rand());
            let v = center.to_vec() - vec3(4.0, 0.2, 0.0);

            let radius = 0.2;
            if v.magnitude() > 0.9 {
                let material: Arc<dyn Material + Send>;
                if random_mat < 0.8 {
                    let albedo = vec3(rand() * rand(), rand() * rand(), rand() * rand());
                    material = Arc::new(Lambertian::new(albedo));
                } else if random_mat < 0.95 {
                    let albedo = vec3(
                        0.5 * (1.0 + rand()),
                        0.5 * (1.0 + rand()),
                        0.5 * (1.0 + rand()),
                    );
                    material = Arc::new(Metal::new(albedo, 0.5 * rand()));
                } else {
                    material = Arc::new(Dielectric::new_glass());
                }
                let sphere = Sphere {
                    center,
                    radius,
                    material: material,
                };
                scene.push(Box::new(sphere));
            }
        }
    }

    scene.push(Box::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new_diamond()),
    }));
    scene.push(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(vec3(0.8, 0.6, 0.5), 0.8)),
    }));
    scene.push(Box::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))),
    }));

    return scene;
}

fn init_thread_pool(threads: Option<usize>) -> usize {
    match threads {
        Some(threads_count) => {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads_count)
                .build_global()
                .unwrap();
            threads_count
        }
        _ => num_cpus::get(),
    }
}

pub fn render(
    output: PathBuf,
    width: u64,
    height: u64,
    samples: u64,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let thread_count = init_thread_pool(threads);

    let mut render_context = RenderContext::new(width, height, samples);
    let scene = get_scene(width, height);

    info!("Using {} threads...", current_num_threads());

    let progress_bar = ProgressBar::new(width * height);
    progress_bar.set_draw_delta(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {percent}% {msg}")
            .progress_chars("##-"),
    );

    render_context.render(&scene, thread_count, &progress_bar);

    render_context.print_stats();

    render_context.save(output);

    let progress_bar = progress_bar.clone();
    progress_bar.finish();

    Ok(())
}
