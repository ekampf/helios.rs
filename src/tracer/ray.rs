use crate::tracer::{Point3f, Vector3f};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn point_at(&self, dist: f64) -> Point3f {
        return self.origin + self.direction * dist;
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Ray origin:({},{},{}) direction:({},{},{}))",
            self.origin.x,
            self.origin.y,
            self.origin.z,
            self.direction.x,
            self.direction.y,
            self.direction.z,
        )
    }
}
