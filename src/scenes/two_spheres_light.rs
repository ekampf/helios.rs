use crate::tracer::bounding_volumes::BVHNode;
use crate::tracer::geometry::Sphere;
use crate::tracer::material::{CheckersTexture, DiffuseLight, Lambertian, SolidTexture};
use crate::tracer::{Camera, Color, RenderOpts, Scene, SceneObjectList, SimpleCamera};
use cgmath::*;
use std::sync::Arc;

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

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidTexture::new(Color::new(
        4f64, 4f64, 4f64,
    )))));
    objects.push(Arc::new(Sphere {
        center: Point3::new(4.0, 3.0, 0.0),
        radius: 1.0,
        //        material: Arc::new(Lambertian::new(noiset)),
        material: light.clone(),
    }));
    let checkers_texture = Arc::new(CheckersTexture::from_colors(
        Color::new(0.2, 0.3, 0.6),
        Color::new(0.99, 0.99, 0.99),
        0.025,
    ));

    let bg_sphere = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(checkers_texture.clone())),
    };
    objects.push(Arc::new(bg_sphere));

    //    let noiset = Arc::new(NoiseTexture::new(5.0));

    objects.push(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(checkers_texture)),
        //        material: light,
    }));

    let bvh = BVHNode::build(objects.objects);
    Scene::new(
        render_options,
        camera,
        Arc::new(bvh),
        Arc::new(Lambertian::from_constant(Color::black())),
    )
}
