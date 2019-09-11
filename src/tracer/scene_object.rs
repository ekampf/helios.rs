use crate::tracer::materials::Material;
use crate::tracer::{Intersectable, Intersection, Point3f, Ray};
use std::sync::Arc;

// TODO: Boundable - returns BoundingBox

pub trait SceneObject: Intersectable + Sync + Send {
    fn get_material(&self, point: Point3f) -> Box<Arc<dyn Material>>;

    fn primitives(&self) -> u64 {
        1
    }
}

pub struct SceneIntersection {
    pub intersection: Intersection,
    pub object: Arc<dyn SceneObject>,
}

pub trait SceneIntersectable: Sync + Send {
    fn intersect(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<SceneIntersection>;
}
