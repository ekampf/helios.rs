use crate::tracer::bounding_volumes::{Boundable, AABB};
use crate::tracer::material::Material;
use crate::tracer::{Intersectable, Intersection, Point3f, Ray, SceneObject, Vector3f};
use cgmath::*;
use std::f64::consts::{FRAC_PI_2, PI};
use std::sync::Arc;

const TWO_PI: f64 = 2.0 * PI;

pub struct Sphere {
    pub center: Point3f,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    fn get_intersection(&self, ray: &Ray, dist: f64) -> Intersection {
        let point = ray.point_at(dist);
        let normal = (point.to_vec() - self.center.to_vec()) / self.radius;

        let uv = self.get_uv(normal);
        Intersection {
            dist,
            point,
            normal,
            uv,
        }
    }

    fn get_uv(&self, normal: Vector3f) -> (f64, f64) {
        //        float phi = atan2(p.z(), p.x());
        //        float theta = asin(p.y());
        //        u = 1-(phi + M_PI) / (2*M_PI);
        //        v = (theta + M_PI/2) / M_PI;
        let phi = normal.z.atan2(normal.x);
        let theta = normal.y.asin();
        let u = 1.0 - ((phi + PI) / (TWO_PI));
        let v = (theta + FRAC_PI_2) / PI;
        (u, v)
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
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            vec3(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        )
    }
}
