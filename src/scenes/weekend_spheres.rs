use crate::tracer::bounding_volumes::BVHNode;
use crate::tracer::geometry::Sphere;
use crate::tracer::material::{
    CheckersTexture, Dielectric, Lambertian, Material, Metal, ScatteredRay,
};
use crate::tracer::{
    Camera, Color, Intersection, Point3f, Ray, RenderOpts, Scene, SceneObjectList, SimpleCamera,
    Vector3f,
};
use cgmath::*;
use rand::Rng;
use std::sync::Arc;

fn rand() -> f64 {
    rand::thread_rng().gen()
}

struct SkyMaterial {}

impl Material for SkyMaterial {
    fn scatter(&self, _ray_in: &Ray, _hit: &Intersection) -> Option<ScatteredRay> {
        None
    }

    fn emitted(&self, ray_in: &Ray, _u: f64, _v: f64, _p: Point3f) -> Vector3f {
        let unit_v = vec3(1.0, 1.0, 1.0);
        let unit_dir = ray_in.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        unit_v * (1.0 - t) + vec3(0.5, 0.7, 1.0) * t
    }
}

fn get_camera(width: u64, height: u64) -> Arc<dyn Camera> {
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

    Arc::new(camera)
}

pub fn get_scene(width: u64, height: u64, samples: u64) -> Scene {
    let camera = get_camera(width, height);
    let render_options = RenderOpts {
        max_depth: 50,
        samples: samples as u32,
    };

    let mut objects = SceneObjectList::new();

    let checkers_texture = Arc::new(CheckersTexture::from_colors(
        Color::new(0.2, 0.3, 0.6),
        Color::new(0.99, 0.99, 0.99),
        0.025,
    ));

    let bg_sphere = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(checkers_texture)),
    };
    objects.push(Arc::new(bg_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let center = Point3::new(a + 0.9 * rand(), 0.2, b + 0.9 * rand());
            let v = center.to_vec() - vec3(4.0, 0.2, 0.0);

            let radius = 0.2;
            if v.magnitude() > 0.9 {
                let material: Arc<dyn Material + Send>;

                let random_mat: f64 = rand();
                if random_mat < 0.8 {
                    // Diffuse material
                    let albedo = Color::new(rand() * rand(), rand() * rand(), rand() * rand());
                    material = Arc::new(Lambertian::from_constant(albedo));
                } else if random_mat < 0.95 {
                    // Metal
                    let albedo = vec3(
                        0.5 * (1.0 + rand()),
                        0.5 * (1.0 + rand()),
                        0.5 * (1.0 + rand()),
                    );
                    material = Arc::new(Metal::new(albedo, 0.5 * rand()));
                } else {
                    // Glass
                    material = Arc::new(Dielectric::new_glass());
                }
                let sphere = Sphere {
                    center,
                    radius,
                    material,
                };
                objects.push(Arc::new(sphere));
            }
        }
    }

    objects.push(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)),
    }));
    objects.push(Arc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new_glass()),
    }));
    objects.push(Arc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::from_constant(Color::new(0.4, 0.2, 0.1))),
    }));

    let bvh = BVHNode::build(objects.objects);
    Scene::new(
        render_options,
        camera,
        Arc::new(bvh),
        Arc::new(SkyMaterial {}),
    )
    //    Scene::new(render_options, camera, Arc::new(objects))
}
