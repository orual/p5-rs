use bevy::prelude::*;
use core::{cmp::PartialOrd, ops::Range};
use num_traits::Num;
use rand::{
    distributions::{uniform::SampleUniform, Open01, Standard},
    prelude::*,
};
use rand_chacha::ChaChaRng;
use statrs::distribution::Normal;

use crate::math::ziggurat_tables;

#[derive(Component)]
pub struct Random {
    rng: ChaChaRng,
    seed: Option<u64>,
}

impl Random {
    pub fn new() -> Self {
        Self {
            rng: ChaChaRng::from_rng(thread_rng()).expect("problem with RNG creation"),
            seed: None,
        }
    }
    pub fn new_seed(seed: u64) -> Self {
        Self {
            rng: ChaChaRng::seed_from_u64(seed),
            seed: Some(seed),
        }
    }

    pub fn random<T>(&mut self, max: Option<T>) -> T
    where
        T: PartialOrd + SampleUniform + Num + Copy,
        Standard: Distribution<T>,
    {
        match max {
            Some(max) => self.rng.gen_range(T::zero()..max),
            _ => self.rng.gen::<T>(),
        }
    }

    pub fn range<T>(&mut self, range: Range<T>) -> T
    where
        T: PartialOrd + SampleUniform,
    {
        self.rng.gen_range(range)
    }

    pub fn seed(&mut self, seed: u64) {
        self.seed = Some(seed);
        self.rng = ChaChaRng::seed_from_u64(seed);
    }

    pub fn normal<T>(&mut self, mean: T, std_dev: T) -> T
    where
        T: Num + Copy + From<f64> + Into<f64>,
    {
        self.rng.sample(Gaussian::new(mean, std_dev))
    }

    pub fn std_normal(&mut self) -> f64 {
        Normal::standard().sample(&mut self.rng)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

struct Gaussian<T> {
    mean: T,
    std_dev: T,
}

impl<T> Gaussian<T> {
    fn new(mean: T, std_dev: T) -> Self {
        Self { mean, std_dev }
    }
}

impl<T> Distribution<T> for Gaussian<T>
where
    T: Num + Copy + From<f64> + Into<f64>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        let sample: f64 = self.mean.into() + self.std_dev.into() * sample_std_normal(rng);
        sample.into()
    }
}

fn sample_std_normal<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    #[inline]
    fn pdf(x: f64) -> f64 {
        (-x * x / 2.0).exp()
    }

    #[inline]
    fn zero_case<R: Rng + ?Sized>(rng: &mut R, u: f64) -> f64 {
        let mut x = 1.0f64;
        let mut y = 0.0f64;
        while -2.0 * y < x * x {
            let x_: f64 = rng.sample(Open01);
            let y_: f64 = rng.sample(Open01);

            x = x_.ln() / ziggurat_tables::ZIG_NORM_R;
            y = y_.ln();
        }
        if u < 0.0 {
            x - ziggurat_tables::ZIG_NORM_R
        } else {
            ziggurat_tables::ZIG_NORM_R - x
        }
    }

    ziggurat(
        rng,
        true,
        &ziggurat_tables::ZIG_NORM_X,
        &ziggurat_tables::ZIG_NORM_F,
        pdf,
        zero_case,
    )
}

// Ziggurat method for sampling a random number based on the ZIGNOR
// variant from Doornik 2005. Code borrowed from
// https://github.com/rust-lang-nursery/rand/blob/master/src/distributions/mod.
// rs#L223
#[inline(always)]
fn ziggurat<R: Rng + ?Sized, P, Z>(
    rng: &mut R,
    symmetric: bool,
    x_tab: ziggurat_tables::ZigTable,
    f_tab: ziggurat_tables::ZigTable,
    mut pdf: P,
    mut zero_case: Z,
) -> f64
where
    P: FnMut(f64) -> f64,
    Z: FnMut(&mut R, f64) -> f64,
{
    const SCALE: f64 = (1u64 << 53) as f64;
    loop {
        let bits: u64 = rng.gen();
        let i = (bits & 0xff) as usize;
        let f = (bits >> 11) as f64 / SCALE;

        // u is either U(-1, 1) or U(0, 1) depending on if this is a
        // symmetric distribution or not.
        let u = if symmetric { 2.0 * f - 1.0 } else { f };
        let x = u * x_tab[i];

        let test_x = if symmetric { x.abs() } else { x };

        // algebraically equivalent to |u| < x_tab[i+1]/x_tab[i] (or u <
        // x_tab[i+1]/x_tab[i])
        if test_x < x_tab[i + 1] {
            return x;
        }
        if i == 0 {
            return zero_case(rng, u);
        }
        // algebraically equivalent to f1 + DRanU()*(f0 - f1) < 1
        if f_tab[i + 1] + (f_tab[i] - f_tab[i + 1]) * rng.gen::<f64>() < pdf(x) {
            return x;
        }
    }
}
