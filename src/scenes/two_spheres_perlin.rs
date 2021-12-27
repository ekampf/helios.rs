use crate::tracer::bounding_volumes::BVHNode;
use crate::tracer::geometry::Sphere;
use crate::tracer::material::{CheckersTexture, Lambertian, Material, NoiseTexture, ScatteredRay};
use crate::tracer::{
    Camera, Color, Intersection, Point3f, Ray, RenderOpts, Scene, SceneObjectList, SimpleCamera,
    Vector3f,
};
use cgmath::*;
use std::sync::Arc;

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

    let cstart = Color::black();
    let cend = Color::new(0.75, 0.0, 0.0);
    let noise_texture = Arc::new(NoiseTexture::new(2.0, 7, 1.0, 0.5, 2.0, cstart, cend));

    objects.push(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(noise_texture)),
    }));

    let bvh = BVHNode::build(objects.objects);
    Scene::new(
        render_options,
        camera,
        Arc::new(bvh),
        Arc::new(SkyMaterial {}),
    )
}
