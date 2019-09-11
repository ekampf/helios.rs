use crate::tracer::geometry::Sphere;
use crate::tracer::materials::{Dielectric, Lambertian, Material, Metal};
use crate::tracer::{Camera, RenderOpts, Scene, SceneObjectList, SimpleCamera};
use cgmath::*;
use rand::Rng;
use std::sync::Arc;

fn rand() -> f64 {
    rand::thread_rng().gen()
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

pub fn get_scene(width: u64, height: u64, samples: u64) -> Scene {
    let camera = get_camera(width, height);
    let render_options = RenderOpts {
        max_depth: 50,
        samples: samples as u32,
    };

    let mut objects = Box::new(SceneObjectList::new());

    let bg_sphere = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))),
    };
    objects.push(Arc::new(bg_sphere));

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
                objects.push(Arc::new(sphere));
            }
        }
    }

    objects.push(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new_diamond()),
    }));
    objects.push(Arc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(vec3(0.8, 0.6, 0.5), 0.8)),
    }));
    objects.push(Arc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))),
    }));

    Scene::new(render_options, camera, objects)
}
