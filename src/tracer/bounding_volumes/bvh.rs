// Test: cargo run --release -- render out/out.png -vvv --width 800 --height 600 --samples 100 -o
// Before:
// ==========================================
//| Rays Cast: 48000000
//| Elapsed Time (s): 94.1374
//| Rays per sec: 509892.76
//==========================================
//==========================================
//| Rays Cast: 48000000
//| Elapsed Time (s): 94.2355
//| Rays per sec: 509362.07
//==========================================
//
//  Avg Rays/sec  = 509,627.415
//
// With BVH:
//==========================================
//| Rays Cast: 48000000
//| Elapsed Time (s): 69.2508
//| Rays per sec: 693132.47
//==========================================
//==========================================
//| Rays Cast: 48000000
//| Elapsed Time (s): 61.9233
//| Rays per sec: 775152.33
//==========================================
//
//  Avg rays/sec = 734,142.4
//
//  TOTAL ~44 percent improve

use crate::tracer::bounding_volumes::{Boundable, BoundingVolume, AABB};
use crate::tracer::{Ray, SceneIntersectable, SceneIntersection, SceneObject};
use rand::prelude::*;
use std::sync::Arc;

pub struct BVHNode {
    bounds: AABB,
    object: Option<Arc<dyn SceneObject>>,
    left: Option<Arc<Self>>,
    right: Option<Arc<Self>>,
}

impl BVHNode {
    pub fn new(
        bounds: AABB,
        object: Option<Arc<dyn SceneObject>>,
        left: Option<Arc<Self>>,
        right: Option<Arc<Self>>,
    ) -> BVHNode {
        BVHNode {
            bounds,
            object,
            left,
            right,
        }
    }

    pub fn build(objects: Vec<Arc<dyn SceneObject>>) -> Self {
        let axis = rand::thread_rng().gen_range(0, 3);
        let mut objects = objects;

        objects.sort_by(|a, b| {
            let left_hit = a.get_bounds().min;
            let right_hit = b.get_bounds().min;

            left_hit[axis].partial_cmp(&right_hit[axis]).unwrap()
        });

        match objects.len() {
            0 => panic!("invalid length"),
            1 => {
                let scene_object = objects.remove(0);
                let bounding_box = scene_object.get_bounds();
                BVHNode::new(bounding_box, Some(scene_object), None, None)
            }
            _ => {
                let (h1, h2) = objects.split_at(objects.len() / 2);
                let h1 = h1.to_vec();
                let h2 = h2.to_vec();

                let left = Self::build(h1);
                let right = Self::build(h2);
                let bounding_box = left.get_bounds().union(&right.get_bounds());

                BVHNode::new(
                    bounding_box,
                    None,
                    Some(Arc::new(left)),
                    Some(Arc::new(right)),
                )
            }
        }
    }
}

impl Boundable for BVHNode {
    fn get_bounds(&self) -> AABB {
        self.bounds
    }
}

impl SceneIntersectable for BVHNode {
    fn intersect(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<SceneIntersection> {
        let box_intersection = self.bounds.fast_intersects(ray, dist_min, dist_max);
        if box_intersection {
            if let Some(object) = &self.object {
                // We reached a leaf node

                let object = object.clone();
                return object
                    .intersects(ray, dist_min, dist_max)
                    .map(|intersection| SceneIntersection {
                        intersection,
                        object,
                    });
            }

            let left = self.left.clone();
            let right = self.right.clone();
            let lsi = match left {
                Some(node) => node.intersect(ray, dist_min, dist_max),
                None => None,
            };
            let rsi = match right {
                Some(node) => node.intersect(ray, dist_min, dist_max),
                None => None,
            };

            return match (lsi, rsi) {
                (Some(lsi), Some(rsi)) => {
                    if lsi.intersection.dist < rsi.intersection.dist {
                        Some(lsi)
                    } else {
                        Some(rsi)
                    }
                }
                (Some(lsi), None) => Some(lsi),
                (None, Some(rsi)) => Some(rsi),
                _ => None,
            };
        }

        None
    }
}
