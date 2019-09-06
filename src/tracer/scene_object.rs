use crate::tracer::Geometry;

pub struct SceneObject {
    pub geometry: Box<dyn Geometry>,
}
