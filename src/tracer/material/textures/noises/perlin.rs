use crate::tracer::Vector3f;
use noise::{NoiseFn, Perlin};

#[derive(Default, Debug, Copy, Clone)]
pub struct PerlinNoise {
    /// Octaves are the number of layers of coherent noise used in the generation of perlin noise.
    /// The default value for this is 8.
    octaves: u32,

    /// Frequency controls the 'width' of the noise. If you imagine noise as hills and valleys, then frequency controls the distance between them.
    /// The default value for this is 1.0
    freq: f64,

    /// Persistence controls how much each octave contributes to the final noise value.
    /// The default value for this is 0.5
    pers: f64,

    /// Lacunarity controls the frequency of each octave in the final noise value.
    /// The default value for this is 2.0
    lacu: f64,

    perlin: Perlin,
}

impl PerlinNoise {
    pub fn new(octaves: u32, freq: f64, pers: f64, lacu: f64) -> PerlinNoise {
        PerlinNoise {
            octaves,
            freq,
            pers,
            lacu,
            perlin: Perlin::new(),
        }
    }

    #[allow(dead_code)]
    pub fn default() -> PerlinNoise {
        PerlinNoise::new(8, 1.0, 0.5, 2.0)
    }

    pub fn noise(&self, p: Vector3f) -> f64 {
        // Based on tutorial from https://flafla2.github.io/2014/08/09/perlinnoise.html
        // Given an octave i we define:
        //   frequency = 2^i
        //   amplitude = persistence * i
        let mut x = p.x;
        let mut y = p.y;
        let mut z = p.z;
        let mut amplitude = self.pers;
        let mut frequency = self.freq;

        let mut total = 0.0;
        let mut max_value = 0.0; // Used to normalize results to [0.0-1.0]

        for _ in 0..self.octaves {
            let perlin_value = self
                .perlin
                .get([x * self.freq, y * self.freq, z * self.freq]); // Returns a -1..1 value

            total += perlin_value * amplitude;
            max_value += amplitude;

            amplitude *= self.pers;
            frequency *= frequency;

            x *= self.lacu;
            y *= self.lacu;
            z *= self.lacu;
        }

        total / max_value
    }

    pub fn noise2(&self, x: f64, y: f64) -> f64 {
        // Given an octave i we define:
        //   frequency = 2^i
        //   amplitude = persistence * i
        let mut x = x;
        let mut y = y;
        let mut amplitude = 1f64;
        let mut frequency = self.freq;

        let mut total = 0.0;
        let mut max_value = 0.0; // Used to normqlize results to [0.0-1.0]

        for _ in 0..self.octaves {
            let perlin_value = self.perlin.get([x * self.freq, y * self.freq]);

            total += perlin_value * amplitude;
            max_value += amplitude;

            amplitude *= self.pers;
            frequency *= frequency;

            x *= self.lacu;
            y *= self.lacu;
        }

        total / max_value
    }
}
