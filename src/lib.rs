mod open_simplex_noise_2d;
mod utils;
mod vector;

use open_simplex_noise_2d::OpenSimplexNoise2D;
use vector::{vec2::Vec2};

pub const PSIZE: i32 = 2048;
const DEFAULT_SEED: u32 = 0;

type PermTable = [i32; PSIZE as usize];

pub struct OpenSimplexNoise {
    perm: PermTable,
}

impl OpenSimplexNoise {
    pub fn new(custom_seed: Option<u32>) -> Self {
        let seed = match custom_seed {
            Some(value) => value,
            None => DEFAULT_SEED,
        };

        Self {
            perm: generate_perm_array(seed),
        }
    }

    pub fn eval_2d(&self, x: f32, y: f32) -> f32 {
        OpenSimplexNoise2D::eval(Vec2::new(x, y), &self.perm)
    }
}

pub trait NoiseEvaluator<T: vector::VecType<f32>> {
    const STRETCH_POINT: T;
    const SQUISH_POINT: T;

    fn eval(point: T, perm: &PermTable) -> f32;
    fn extrapolate(grid: T, delta: T, perm: &PermTable) -> f32;
}

fn generate_perm_array(seed: u32) -> PermTable {
    let mut perm: PermTable = [0; PSIZE as usize];

    let mut source: Vec<i32> = (0..PSIZE).collect();

    let seed: i128 = (seed as i128 * 6_364_136_223_846_793_005) + 1_442_695_040_888_963_407;
    for i in (0..PSIZE).rev() {
        let mut r = ((seed + 31) % (i as i128 + 1)) as i32;
        if r < 0 {
            r += i + 1;
        }
        perm[i as usize] = source[r as usize];
        source[r as usize] = source[i as usize];
    }

    perm
}
