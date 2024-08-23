use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

pub const PERLIN_DEFAULT_LOD: u16 = 4;
pub const PERLIN_DEFAULT_FALLOFF: f64 = 0.5;

#[derive(Component)]
pub struct Noise {
    perlin: Perlin,
    amp_falloff: f64,
    octaves: u16,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            perlin: Perlin::default(),
            amp_falloff: PERLIN_DEFAULT_FALLOFF,
            octaves: PERLIN_DEFAULT_LOD,
        }
    }

    pub fn new_seed(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            amp_falloff: PERLIN_DEFAULT_FALLOFF,
            octaves: PERLIN_DEFAULT_LOD,
        }
    }

    pub fn seed(&mut self, seed: u32) {
        self.perlin = self.perlin.set_seed(seed);
    }

    pub fn tune(&mut self, lod: u16, falloff: f64) {
        if lod > 0 {
            self.octaves = lod;
        }
        if falloff > 0.0 {
            self.amp_falloff = falloff;
        }
    }

    pub fn noise(&mut self, t: f64) -> f64 {
        let mut total = 0.0;
        let mut max = 0.0;
        let mut amp = 1.0;
        let mut freq = 1.0;
        for _i in 0..self.octaves {
            total += self.perlin.get([t * freq]) * amp;
            max += amp;
            amp *= self.amp_falloff;
            freq *= 2.0;
        }
        total / max
    }

    pub fn noise2d(&mut self, x: f64, t: f64) -> f64 {
        let mut total = 0.0;
        let mut max = 0.0;
        let mut amp = 1.0;
        let mut freq = 1.0;
        for _i in 0..self.octaves {
            total += self.perlin.get([x * freq, t * freq]) * amp;
            max += amp;
            amp *= self.amp_falloff;
            freq *= 2.0;
        }
        total / max
    }

    pub fn noise3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut total = 0.0;
        let mut max = 0.0;
        let mut amp = 1.0;
        let mut freq = 1.0;
        for _i in 0..self.octaves {
            total += self.perlin.get([x * freq, y * freq, z * freq]) * amp;
            max += amp;
            amp *= self.amp_falloff;
            freq *= 2.0;
        }
        total / max
    }

    pub fn noise4d(&mut self, x: f64, y: f64, z: f64, t: f64) -> f64 {
        let mut total = 0.0;
        let mut max = 0.0;
        let mut amp = 1.0;
        let mut freq = 1.0;
        for _i in 0..self.octaves {
            total += self.perlin.get([x * freq, y * freq, z * freq, t * freq]) * amp;
            max += amp;
            amp *= self.amp_falloff;
            freq *= 2.0;
        }
        total / max
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self::new()
    }
}
