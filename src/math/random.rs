use bevy::prelude::*;
use rand::{
    distributions::{Standard, Uniform},
    prelude::*,
};
use rand_chacha::ChaChaRng;

#[derive(Component)]
pub struct Random {
    rng: ChaChaRng,
    seed: Option<u64>,
}

impl Random {
    pub fn new(seed: Option<u64>) -> Self {
        match seed {
            Some(seed) => Self {
                rng: ChaChaRng::seed_from_u64(seed),
                seed: Some(seed),
            },
            None => Self {
                rng: ChaChaRng::from_rng(thread_rng()),
                seed: None,
            },
        }
    }

    pub fn random<T>(&mut self, max: Option<T>) -> T {
        match max {
            Some(val) => self.rng.gen_range(0 as T..max),
            None => self.rng.sample(Standard),
        }
    }

    pub fn random_from<T>(&mut self, range: Range<T>) -> T {
        self.rng.gen_range(range)
    }
}
