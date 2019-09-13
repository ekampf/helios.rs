use crate::tracer::material::Material;
use crate::tracer::{Camera, Ray, SceneIntersectable, SceneIntersection};
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct RenderOpts {
    pub max_depth: u32,
    pub samples: u32,
}

pub struct Scene {
    pub options: RenderOpts,
    pub camera: Arc<dyn Camera>,
    pub objects: Arc<dyn SceneIntersectable>,
    pub background: Arc<dyn Material>,
}

impl Scene {
    pub fn new(
        options: RenderOpts,
        camera: Arc<dyn Camera>,
        objects: Arc<dyn SceneIntersectable>,
        background: Arc<dyn Material>,
    ) -> Scene {
        Scene {
            options,
            camera,
            objects,
            background,
        }
    }
}

impl SceneIntersectable for Scene {
    fn intersect(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<SceneIntersection> {
        self.objects.intersect(ray, dist_min, dist_max)
    }
}
