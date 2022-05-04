use super::utils;
use super::vector::{vec2::Vec2, VecMethods};
use super::NoiseEvaluator;
use super::PermTable;

const STRETCH: f32 = -0.211_324_87; // (1 / sqrt(2 + 1) - 1) / 2
const SQUISH: f32 = 0.366_025_42; // (sqrt(2 + 1) - 1) / 2

const NORMALIZING_SCALAR: f32 = 47.0;

const GRAD_TABLE: [Vec2<f32>; 8] = [
    Vec2::new(5.0, 2.0),
    Vec2::new(2.0, 5.0),
    Vec2::new(-5.0, 2.0),
    Vec2::new(-2.0, 5.0),
    Vec2::new(5.0, -2.0),
    Vec2::new(2.0, -5.0),
    Vec2::new(-5.0, -2.0),
    Vec2::new(-2.0, -5.0),
];

pub struct OpenSimplexNoise2D {}

impl NoiseEvaluator<Vec2<f32>> for OpenSimplexNoise2D {
    const STRETCH_POINT: Vec2<f32> = Vec2::new(STRETCH, STRETCH);
    const SQUISH_POINT: Vec2<f32> = Vec2::new(SQUISH, SQUISH);

    fn eval(input: Vec2<f32>, perm: &PermTable) -> f32 {
        let stretch: Vec2<f32> = input + (Self::STRETCH_POINT * input.sum());
        let grid = stretch.map(utils::floor).map(utils::to_f32);

        let squashed: Vec2<f32> = grid + (Self::SQUISH_POINT * grid.sum());
        let ins = stretch - grid;
        let origin = input - squashed;

        OpenSimplexNoise2D::get_value(grid, origin, ins, perm)
    }

    fn extrapolate(grid: Vec2<f32>, delta: Vec2<f32>, perm: &PermTable) -> f32 {
        let point = GRAD_TABLE[Self::get_grad_table_index(grid, perm)];
        point.x * delta.x + point.y * delta.y
    }
}

impl OpenSimplexNoise2D {
    fn get_value(grid: Vec2<f32>, origin: Vec2<f32>, ins: Vec2<f32>, perm: &PermTable) -> f32 {
        let contribute = |x, y| -> f32 {
            utils::contribute::<OpenSimplexNoise2D, Vec2<f32>>(Vec2::new(x, y), origin, grid, perm)
        };

        let value = contribute(1.0, 0.0)
            + contribute(0.0, 1.0)
            + Self::evaluate_inside_triangle(ins, contribute);

        value / NORMALIZING_SCALAR
    }

    fn evaluate_inside_triangle(ins: Vec2<f32>, contribute: impl Fn(f32, f32) -> f32) -> f32 {
        let in_sum = ins.sum();
        let factor_point = match in_sum {
            x if x <= 1.0 => Vec2::new(0.0, 0.0),
            _ => Vec2::new(1.0, 1.0),
        };
        Self::evaluate_inside_triangle_at(factor_point, in_sum, ins, contribute)
    }

    fn evaluate_inside_triangle_at(
        factor_point: Vec2<f32>,
        in_sum: f32,
        ins: Vec2<f32>,
        contribute: impl Fn(f32, f32) -> f32,
    ) -> f32 {
        let zins = 1.0 + factor_point.x - in_sum;
        let point = if zins > ins.x || zins > ins.y {
            // (0, 0) is one of the closest two triangular vertices
            if ins.x > ins.y {
                Vec2::new(1.0 + factor_point.x, -1.0 + factor_point.y)
            } else {
                Vec2::new(-1.0 + factor_point.x, 1.0 + factor_point.y)
            }
        } else {
            // (1, 0) and (0, 1) are the closest two vertices.
            Vec2::new(1.0 - factor_point.x, 1.0 - factor_point.y)
        };

        contribute(0.0 + factor_point.x, 0.0 + factor_point.y) + contribute(point.x, point.y)
    }

    fn get_grad_table_index(grid: Vec2<f32>, perm: &PermTable) -> usize {
        let index0 = ((perm[(grid.x as i32 & 0xFF) as usize] + grid.y as i32) & 0xFF) as usize;
        ((perm[index0] & 0x0E) >> 1) as usize
    }
}
