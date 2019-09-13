//use crate::tracer::material::Texture;
//use crate::tracer::{Point3f, Vector3f};
//use cgmath::*;

/// Base trait for noise functions.
///
/// A noise function is a object that calculates and outputs a value given a
/// n-Dimensional input value, where n is (2,3,4).
///
/// Each type of noise function uses a specific method to calculate an output
/// value. Some of these methods include:
///
/// * Calculating a value using a coherent-noise function or some other
///     mathematical function.
/// * Mathematically changing the output value from another noise function
///     in various ways.
/// * Combining the output values from two noise functions in various ways.
pub trait NoiseFn<T> {
    fn get(&self, point: T) -> f64;
}

//pub struct NoiseTexture {
//    scale: f64,
//    noise: Perlin,
//}
//
//impl NoiseTexture {
//    pub fn new(scale: f64) -> NoiseTexture {
//        NoiseTexture {
//            scale,
//            noise: Perlin::new(),
//        }
//    }
//}
//
//impl Texture for NoiseTexture {
//    fn texture_value(&self, _u: f64, _v: f64, p: Point3f) -> Vector3f {
//        let sp = p * self.scale;
//        let v1 = vec3(1.0, 1.0, 1.0);
//        v1 * self.noise.get([sp.x, sp.y, sp.z])
//    }
//}
