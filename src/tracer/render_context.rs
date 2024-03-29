use crate::tracer::{Color, Point3f, Ray, Scene, SceneIntersectable};
use image::ImageBuffer;
use indicatif::ProgressBar;
use itertools::Itertools;
use log::info;
use rand::prelude::*;
use rayon::prelude::*;
use std::ops::Range;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug)]
pub struct RenderContext {
    pub width: u64,
    pub height: u64,

    pub pixels: Vec<Color>,

    // Some stats
    pub rays_cast: u64,
    pub start_time: Instant,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderTask {
    pub from_x: u64,
    pub from_y: u64,
    pub to_x: u64,
    pub to_y: u64,
    pub width: u64,
    pub height: u64,
}

pub struct RenderResult {
    pub pixels: Vec<Color>,
    pub rays_cast: u64,
}

impl RenderContext {
    pub fn new(width: u64, height: u64) -> RenderContext {
        let total_pixels = width * height;
        RenderContext {
            width,
            height,
            pixels: vec![Color::black(); total_pixels as usize],
            rays_cast: 0,
            start_time: Instant::now(),
        }
    }

    pub fn print_stats(&self) {
        let elapsed = self.start_time.elapsed();

        println!();
        println!("==========================================");
        println!("| Rays Cast: {}", self.rays_cast);
        println!("| Elapsed Time (s): {:.4}\n", elapsed.as_secs_f64());
        println!(
            "| Rays per sec: {:.2}\n",
            self.rays_cast as f64 / elapsed.as_secs_f64()
        );
        println!("==========================================");
    }

    pub fn get_stats(&self) -> (u64, f64, f64) {
        let elapsed = self.start_time.elapsed();
        (
            self.rays_cast,
            elapsed.as_secs_f64(),
            self.rays_cast as f64 / elapsed.as_secs_f64(),
        )
    }

    pub fn get_stats_message(&self) -> String {
        let (rays_cast, elapsed, rays_per_sec) = self.get_stats();
        format!(
            "Rays Cast: {} | Elapsed Time (s): {:.4} | Rays per sec: {:.2}",
            rays_cast, elapsed, rays_per_sec
        )
    }

    fn get_tasks(&self, n: usize) -> Vec<RenderTask> {
        let mut v = Vec::with_capacity(n as usize);
        let width = self.width as usize;

        let chunk_size = (width as f64 / n as f64).ceil() as usize;

        for mut wc in &(0..width).into_iter().chunks(chunk_size) {
            let chunk_size = chunk_size as u64;
            let from_x: u64 = wc.next().unwrap() as u64;
            let to_x = (from_x + chunk_size).min(self.width);

            let task = RenderTask {
                from_x,
                to_x,
                from_y: 0,
                to_y: self.height,
                width: self.width,
                height: self.height,
            };
            v.push(task);
        }

        v
    }

    pub fn set_pixel(&mut self, x: u64, y: u64, color: Color) {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx] += color;
    }

    pub fn get_pixel(&self, x: u64, y: u64) -> Color {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx]
    }

    pub fn apply_render_result(&mut self, task: &RenderTask, result: &RenderResult) {
        let mut idx = 0;
        for y in task.yrange() {
            for x in task.xrange() {
                self.set_pixel(x, y, result.pixels[idx]);
                idx += 1
            }
        }

        self.rays_cast += result.rays_cast;
    }

    pub fn render(&mut self, scene: &Scene, threads: usize, pb: Option<&ProgressBar>) {
        let render_tasks = self.get_tasks(threads * 3);
        info!("Render tasks: {}", render_tasks.len());

        let rcm = Arc::new(Mutex::new(self));

        render_tasks.into_par_iter().for_each_with(rcm, |rcm, t| {
            let result = t.render(scene, pb);

            let mut rc = rcm.lock().unwrap();
            rc.apply_render_result(&t, &result);

            if let Some(pb) = pb {
                let message = rc.get_stats_message();
                pb.set_message(&*message);
            }
        });
    }

    pub fn save(&self, output: &Path) {
        let img = ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let x = x as u64;
            let y = y as u64;

            let mut color = self.get_pixel(x, y);
            color = color.sqrt();
            color *= 255.99f64;

            image::Rgb([color.red as u8, color.green as u8, color.blue as u8])
        });

        match img.save(output) {
            Ok(_) => println!("Saved to file!"),
            Err(error) => println!("Oh noes: {}", error),
        }
    }
}

impl RenderTask {
    pub fn xrange_width(&self) -> u64 {
        self.to_x - self.from_x
    }

    pub fn yrange_height(&self) -> u64 {
        self.to_y - self.from_y
    }

    pub fn task_pixels_count(&self) -> u64 {
        self.xrange_width() * self.yrange_height()
    }

    pub fn xrange(&self) -> Range<u64> {
        self.from_x..self.to_x
    }

    pub fn yrange(&self) -> Range<u64> {
        self.from_y..self.to_y
    }

    pub fn render(&self, scene: &Scene, pb: Option<&ProgressBar>) -> RenderResult {
        let total_pixels = self.task_pixels_count();
        let mut pixels: Vec<Color> = Vec::with_capacity(total_pixels as usize);
        let mut rays_cast = 0;

        for y in self.yrange() {
            for x in self.xrange() {
                let (cast, pixel) = self.render_pixel(x, y, scene);
                pixels.push(pixel);
                rays_cast += cast;

                if let Some(pb) = pb {
                    pb.inc(1);
                }
            }
        }

        RenderResult { pixels, rays_cast }
    }

    fn render_pixel(&self, x: u64, y: u64, scene: &Scene) -> (u64, Color) {
        let mut color = Color::black();
        let mut rays_count = 0;
        let mut rng = rand::thread_rng();

        let x = x as f64;
        let y = y as f64;

        let samples = scene.options.samples;
        let width = self.width as f64;
        let height = self.height as f64;

        for _ in 0..samples {
            let su: f64 = rng.gen();
            let sv: f64 = rng.gen();

            let u = (x + su) / width;
            let v = (height - y + sv) / height;

            let ray = scene.camera.get_ray(u, v);
            let color_sample = RenderTask::cast_ray(&ray, scene, &mut rng, 0);

            color += color_sample;
            rays_count += 1
        }

        color /= samples;

        (rays_count, color)
    }

    fn cast_ray(ray: &Ray, scene: &Scene, rng: &mut ThreadRng, depth: u32) -> Color {
        let maybe_intersection = scene.intersect(ray, 0.001, std::f64::MAX);

        if let Some(intersection) = maybe_intersection {
            let material = intersection
                .object
                .get_material(intersection.intersection.point);

            let intersection = intersection.intersection;
            let (u, v) = intersection.uv;
            let emitted = material.emitted(ray, u, v, intersection.point);

            if depth < scene.options.max_depth {
                if let Some(ref scatter) = material.scatter(ray, &intersection) {
                    let attenution = scatter.attenuation;
                    let color = RenderTask::cast_ray(&scatter.ray, scene, rng, depth + 1);
                    return (color * attenution) + emitted;
                }
                return Color::from_vec3f(emitted);
            }

            return Color::from_vec3f(emitted);
        }

        Color::from_vec3f(
            scene
                .background
                .emitted(ray, 0.0, 0.0, Point3f::new(0.0, 0.0, 0.0)),
        )
    }
}
