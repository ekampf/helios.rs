extern crate cgmath;
extern crate image;
extern crate itertools;
extern crate rand;
extern crate time;

use crate::tracer::{Color, Ray, Scene};
use cgmath::*;
use image::ImageBuffer;
use indicatif::ProgressBar;
use itertools::Itertools;
use log::info;
use rand::prelude::*;
use rayon::prelude::*;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct RenderContext {
    pub width: u64,
    pub height: u64,
    pub samples: u64,

    pub pixels: Vec<Color>,

    // Some stats
    pub rays_cast: u64,
    pub start_time: f64,
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
    pub fn new(width: u64, height: u64, samples: u64) -> RenderContext {
        let total_pixels = width * height;
        RenderContext {
            width,
            height,
            samples,
            pixels: vec![Color::black(); total_pixels as usize],
            rays_cast: 0,
            start_time: time::precise_time_s(),
        }
    }

    pub fn print_stats(&self) {
        let elapsed = time::precise_time_s() - self.start_time;

        print!("\n==========================================\n");
        print!("| Rays Cast: {}\n", self.rays_cast);
        print!("| Elapsed Time (s): {:.4}\n", elapsed);
        print!("| Rays per sec: {:.2}\n", self.rays_cast as f64 / elapsed);
        print!("==========================================\n");
    }

    pub fn get_stats(&self) -> (u64, f64, f64) {
        let elapsed = time::precise_time_s() - self.start_time;
        (self.rays_cast, elapsed, self.rays_cast as f64 / elapsed)
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
            let from_x: u64 = wc.nth(0).unwrap() as u64;
            let to_x = (from_x + chunk_size).min(self.width);
            //            info!("Generating chunk - from_x: {}, to_x: {}", from_x, to_x);

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

    pub fn render(&mut self, scene: &Scene, threads: usize, pb: &ProgressBar) {
        let render_tasks = self.get_tasks(threads * 3);
        info!("Render tasks: {}", render_tasks.len());

        let samples = self.samples;

        let rcm = Arc::new(Mutex::new(self));

        render_tasks.into_par_iter().for_each_with(rcm, |rcm, t| {
            let result = t.render(&scene, samples, pb);

            let mut rc = rcm.lock().unwrap();
            rc.apply_render_result(&t, &result);

            let message = rc.get_stats_message();
            pb.set_message(&*message);
        });
    }

    pub fn save(&self, output: PathBuf) {
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

    pub fn render(&self, scene: &Scene, samples: u64, pb: &ProgressBar) -> RenderResult {
        let total_pixels = self.task_pixels_count();
        let mut pixels: Vec<Color> = Vec::with_capacity(total_pixels as usize);
        let mut rays_cast = 0;

        for y in self.yrange() {
            for x in self.xrange() {
                let (cast, pixel) = self.render_pixel(x, y, samples, &scene);
                pixels.push(pixel);
                rays_cast += cast;
                pb.inc(1);
            }
        }

        RenderResult { pixels, rays_cast }
    }

    fn render_pixel(&self, x: u64, y: u64, samples: u64, scene: &Scene) -> (u64, Color) {
        let mut color = Color::black();
        let mut rays_count = 0;
        let mut rng = rand::thread_rng();

        let x = x as f64;
        let y = y as f64;

        let width = self.width as f64;
        let height = self.height as f64;

        for _ in 0..samples {
            let su: f64 = rng.gen();
            let sv: f64 = rng.gen();

            let u = (x + su) / width;
            let v = (height - y + sv) / height;

            let ray = scene.camera.get_ray(u, v);
            let color_sample = RenderTask::cast_ray(&ray, &scene, &mut rng, 0);

            color += color_sample;
            rays_count += 1
        }

        color /= samples;

        (rays_count, color)
    }

    fn cast_ray(ray: &Ray, scene: &Scene, rng: &mut ThreadRng, depth: u32) -> Color {
        let maybe_intersection = scene.intersect(ray, 0.001, std::f64::MAX);

        let unit_v = vec3(1.0, 1.0, 1.0);

        if let Some(intersection) = maybe_intersection {
            let material = intersection
                .object
                .geometry
                .get_material(intersection.intersection.point);

            if depth < 50 {
                if let Some(ref scatter_ray) = material.scatter(ray, &intersection.intersection) {
                    let attenution = scatter_ray.attenuation;
                    let color = RenderTask::cast_ray(&scatter_ray.ray, scene, rng, depth + 1);
                    return color * attenution;
                }
            }

            return Color::black();
        }

        // TODO: this renders sky. Should refactor this out to the Scene object
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        return Color::from_vec3f(unit_v * (1.0 - t) + vec3(0.5, 0.7, 1.0) * t);
    }
}
