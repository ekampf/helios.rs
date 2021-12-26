use crate::tracer::{Point3f, Ray, Vector3f};
use std::cmp;
use std::fmt;

pub trait Intersectable: Sync {
    fn intersects(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Intersection>;
}

#[derive(Clone)]
pub struct Intersection {
    pub dist: f64,
    pub point: Point3f,
    pub normal: Vector3f,
    pub uv: (f64, f64),
}

impl cmp::PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.point == other.point
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Intersection point:({},{},{}), dist:{} normal:({},{},{}))",
            self.point.x,
            self.point.y,
            self.point.z,
            self.dist,
            self.normal.x,
            self.normal.y,
            self.normal.z,
        )
    }
}
