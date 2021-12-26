use crate::tracer::bounding_volumes::BoundingVolume;
use crate::tracer::{Point3f, Ray, Vector3f};
use cgmath::*;

/// Axis-aligned Bounding Box
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vector3f,
    pub max: Vector3f,
}

impl AABB {
    pub fn new(min: Vector3f, max: Vector3f) -> AABB {
        AABB { min, max }
    }

    pub fn union(&self, other: &AABB) -> AABB {
        let min = vec3(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        let max = vec3(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );
        AABB { min, max }
    }

    #[allow(dead_code)]
    pub fn contains_point(self, p: Vector3f) -> bool {
        if p.x < self.min.x || p.x > self.max.x {
            return false;
        }
        if p.y < self.min.y || p.y > self.max.y {
            return false;
        }
        if p.z < self.min.z || p.z > self.max.z {
            return false;
        }

        true
    }
}

#[inline]
fn find_min_max(min: &Vector3f, max: &Vector3f, ro: &Point3f, invrd: &Vector3f) -> (f64, f64) {
    let t1 = (min.x - ro.x) * invrd.x;
    let t2 = (max.x - ro.x) * invrd.x;
    let t3 = (min.y - ro.y) * invrd.y;
    let t4 = (max.y - ro.y) * invrd.y;
    let t5 = (min.z - ro.z) * invrd.z;
    let t6 = (max.z - ro.z) * invrd.z;

    let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
    let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
    return (tmin, tmax);
}

impl BoundingVolume for AABB {
    fn fast_intersects(&self, ray: &Ray, _dist_min: f64, _dist_max: f64) -> bool {
        let (tmin, tmax) = find_min_max(
            &self.min,
            &self.max,
            &ray.origin,
            &ray.get_inverse_direction(),
        );

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        if tmax < 0. {
            return false;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracer::{Ray, Vector3f};

    #[test]
    fn test_fast_intersects() {
        let b = AABB::new(vec3(1., 1., 1.), vec3(2., 2., 2.));
        let r = Ray::new(Point3f::new(0.0, 0.0, 0.0), Vector3f::new(1.4, 1.0, 1.1));
        let i = b.fast_intersects(&r, 0.0001, std::f64::MAX);
        assert!(i);

        let b = AABB::new(vec3(1., 1., 1.), vec3(2., 2., 2.));
        let r = Ray::new(Point3f::new(0.0, 0.0, 0.0), Vector3f::new(1.4, 1.0, -1.1));
        let i = b.fast_intersects(&r, 0.0001, std::f64::MAX);
        assert!(!i);
    }
}
