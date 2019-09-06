use cgmath::*;
use rand::prelude::*;

pub type Point3f = Point3<f64>;
pub type Vector3f = Vector3<f64>;

pub fn random_point_on_unit_sphere() -> Vector3f {
    // Pick a random point in the unit cube where x,y,z all range from -1 to +1.
    // Reject and try again if point is outside of sphere
    let mut rng = rand::thread_rng();
    let one = vec3(1_f64, 1_f64, 1_f64);

    loop {
        let random_vec = vec3(rng.gen(), rng.gen(), rng.gen());
        let p = (random_vec * 2_f64) - one;

        if p.magnitude2() >= 1_f64 {
            return p;
        }
    }
}
