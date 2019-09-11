use crate::tracer::bounding_volumes::AABB;
use crate::tracer::Ray;

pub trait BoundingVolume {
    fn fast_intersects(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> bool;
}

pub trait Boundable {
    fn get_bounds(&self) -> AABB;
}
