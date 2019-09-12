use crate::tracer::{Point3f, Vector3f};

pub trait Texture: Sync + Send {
    fn texture_value(&self, u: f64, v: f64, p: Point3f) -> Vector3f;
}
