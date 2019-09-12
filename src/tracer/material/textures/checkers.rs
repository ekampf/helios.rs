use crate::tracer::material::{SolidTexture, Texture};
use crate::tracer::{Color, Point3f, Vector3f};
use std::sync::Arc;

pub struct CheckersTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
    pub scale: f64,
}

impl CheckersTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, scale: f64) -> CheckersTexture {
        CheckersTexture { odd, even, scale }
    }

    pub fn from_colors(odd: Color, even: Color, scale: f64) -> CheckersTexture {
        let odd = Arc::new(SolidTexture::new(odd));
        let even = Arc::new(SolidTexture::new(even));
        CheckersTexture { odd, even, scale }
    }
}

impl Texture for CheckersTexture {
    fn texture_value(&self, u: f64, v: f64, p: Point3f) -> Vector3f {
        let pt = p * 10.0;
        let sines = pt.x.sin() * pt.y.sin() * pt.z.sin();

        if sines < 0.0 {
            return self.odd.texture_value(u, v, p);
        }

        self.even.texture_value(u, v, p)
        //        let s = (u % self.scale).abs();
        //        let t = (v % self.scale).abs();
        //        let half = self.scale / 2.0;
        //
        //        if s > half && t < half || s < half && t > half {
        //            return self.odd.texture_value(u, v, p);
        //        }
        //
        //        self.even.texture_value(u, v, p)
    }
}
