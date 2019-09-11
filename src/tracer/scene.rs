use crate::tracer::{Camera, Ray, SceneIntersectable, SceneIntersection};

#[derive(Copy, Clone, Debug)]
pub struct RenderOpts {
    pub max_depth: u32,
    pub samples: u32,
}

pub struct Scene {
    pub options: RenderOpts,
    pub camera: Box<dyn Camera>,
    pub objects: Box<dyn SceneIntersectable>,
}

impl Scene {
    pub fn new(
        options: RenderOpts,
        camera: Box<dyn Camera>,
        objects: Box<dyn SceneIntersectable>,
    ) -> Scene {
        Scene {
            options,
            camera,
            objects,
        }
    }
}

impl SceneIntersectable for Scene {
    fn intersect(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<SceneIntersection> {
        self.objects.intersect(ray, dist_min, dist_max)
    }
}
