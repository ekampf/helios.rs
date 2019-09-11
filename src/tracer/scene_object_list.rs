use crate::tracer::{Intersection, Ray, SceneIntersectable, SceneIntersection, SceneObject};
use std::sync::Arc;

pub struct SceneObjectList {
    pub objects: Vec<Arc<dyn SceneObject>>,
}

impl SceneObjectList {
    pub fn new() -> SceneObjectList {
        SceneObjectList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Arc<dyn SceneObject>) {
        self.objects.push(object);
    }
}

impl SceneIntersectable for SceneObjectList {
    fn intersect(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<SceneIntersection> {
        let mut closest: Option<Intersection> = None;
        let mut closest_obj: Option<Arc<dyn SceneObject>> = None;
        let mut closest_dist = dist_max;

        for obj in self.objects.iter() {
            let maybe_intersect = obj.intersects(ray, dist_min, closest_dist);
            if let Some(ref inter) = maybe_intersect {
                closest_dist = inter.dist;
                closest = maybe_intersect;
                closest_obj = Some(obj.clone());
            }
        }

        if let Some(intersection) = closest {
            let object = closest_obj.unwrap();
            return Some(SceneIntersection {
                intersection,
                object,
            });
        }

        None
    }
}
