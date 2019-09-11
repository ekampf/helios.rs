use crate::tracer::bounding_volumes::{Boundable, AABB};
use crate::tracer::materials::Material;
use crate::tracer::{Intersectable, Intersection, Point3f, Ray, SceneObject};
use cgmath::*;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3f,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    fn get_intersection(&self, ray: &Ray, dist: f64) -> Intersection {
        let point = ray.point_at(dist);
        let normal = (point.to_vec() - self.center.to_vec()) / self.radius;
        Intersection {
            dist,
            point,
            normal,
        }
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c);
        if discriminant < 0f64 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / a;
        if t < dist_max && t > dist_min {
            return Some(self.get_intersection(ray, t));
        }

        let t = (-b + discriminant.sqrt()) / a;
        if t < dist_max && t > dist_min {
            return Some(self.get_intersection(ray, t));
        }

        return None;
    }
}

impl SceneObject for Sphere {
    fn get_material(&self, _point: Point3f) -> Box<Arc<dyn Material>> {
        Box::new(self.material.clone())
    }
}

impl Boundable for Sphere {
    fn get_bounds(&self) -> AABB {
        AABB::new(
            vec3(
                &self.center.x - &self.radius,
                &self.center.y - &self.radius,
                &self.center.z - &self.radius,
            ),
            vec3(
                &self.center.x + &self.radius,
                &self.center.y + &self.radius,
                &self.center.z + &self.radius,
            ),
        )
    }
}
