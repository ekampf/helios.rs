use crate::tracer::{Point3f, Vector3f};
use std::cmp;
use std::fmt;

#[derive(Clone)]
pub struct Intersection {
    pub dist: f64,
    pub point: Point3f,
    pub normal: Vector3f,
}

impl cmp::PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        &self.point == &other.point
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
