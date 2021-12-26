use crate::tracer::{Point3f, Vector3f};
use cgmath::*;
use std::cell::RefCell;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,

    inverse_direction: RefCell<Option<Vector3f>>,
}

impl Ray {
    pub fn new(origin: Point3f, direction: Vector3f) -> Ray {
        Ray {
            origin,
            direction,
            inverse_direction: RefCell::new(None),
        }
    }

    pub fn point_at(&self, dist: f64) -> Point3f {
        self.origin + self.direction * dist
    }

    pub fn get_inverse_direction(&self) -> Vector3f {
        self.inverse_direction
            .borrow_mut()
            .get_or_insert_with(|| {
                vec3(
                    1f64 / self.direction.x,
                    1f64 / self.direction.y,
                    1f64 / self.direction.z,
                )
            })
            .clone()
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
