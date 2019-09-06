use crate::tracer::{Camera, Geometry, Intersection, Ray, SceneObject};
use std::sync::Arc;

pub struct Scene {
    pub camera: Box<dyn Camera>,
    pub objects: Vec<Arc<SceneObject>>,
}

pub struct SceneIntersection {
    pub intersection: Intersection,
    pub object: Arc<SceneObject>,
}

impl Scene {
    pub fn new(camera: Box<dyn Camera>) -> Scene {
        Scene {
            camera,
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, geometry: Box<dyn Geometry + Sync + Send>) {
        self.objects.push(Arc::new(SceneObject { geometry }));
    }

    pub fn intersect(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<SceneIntersection> {
        let mut closest: Option<Intersection> = None;
        let mut closest_obj: Option<Arc<SceneObject>> = None;
        let mut closest_dist = dist_max;

        for obj in self.objects.iter() {
            let maybe_intersect = obj.geometry.intersects(ray, dist_min, closest_dist);
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
