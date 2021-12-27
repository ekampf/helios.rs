use crate::tracer::{random_in_unit_sphere, Point3f, Ray, Vector3f};
use cgmath::*;
use std::f64::consts::PI;
use std::fmt;

pub trait Camera: Sync + Send {
    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleCamera {
    pub origin: Point3f,
    pub lower_left_corner: Vector3f,
    pub horizontal: Vector3f,
    pub vertical: Vector3f,

    u: Vector3f,
    v: Vector3f,
    w: Vector3f,
    lens_radius: f64,
}

impl SimpleCamera {
    /// * `vfof` - top to bottom in degrees
    /// * `aspect` -
    pub fn new(
        look_from: Point3f,
        look_at: Vector3f,
        up: Vector3f,
        vfof: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> SimpleCamera {
        let lens_radius = aperture / 2_f64;
        let theta = vfof * PI / 180_f64;
        let half_height = (theta / 2_f64).tan();
        let half_width = aspect * half_height;

        let look_from_vec = look_from.to_vec();
        let w = (look_from_vec - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let lower_left_corner = look_from_vec
            - half_width * focus_dist * u
            - half_height * focus_dist * v
            - focus_dist * w;
        let horizontal = u * 2_f64 * half_width * focus_dist;
        let vertical = v * 2_f64 * half_height * focus_dist;

        SimpleCamera {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }
}

impl Camera for SimpleCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = (u * rd.x) + (v * rd.y);
        let offset_vec = vec3(offset, offset, offset);

        let origin = self.origin.to_vec() + offset_vec;

        Ray::new(
            Point3::new(origin.x, origin.y, origin.z),
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v)
                - self.origin.to_vec()
                - offset_vec,
        )
    }
}

impl fmt::Display for SimpleCamera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(SimpleCamera origin:{:?} lower_left_corner:{:?}, horizontal:{:?}, vertical:{:?}, u:{:?}, v:{:?}, w:{:?}, lens radius: {}",
            self.origin,
            self.lower_left_corner,
            self.horizontal,
            self.vertical,
            self.u,
            self.v,
            self.w,
            self.lens_radius,
        )
    }
}
