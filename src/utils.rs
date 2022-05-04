use super::vector::VecType;
use super::NoiseEvaluator;
use super::PermTable;

pub fn contribute<NoiseEvaluatorType: NoiseEvaluator<Vec>, Vec: VecType<f32>>(
    delta: Vec,
    origin: Vec,
    grid: Vec,
    perm: &PermTable,
) -> f32 {
    let shifted: Vec = origin - delta - NoiseEvaluatorType::SQUISH_POINT * delta.sum();
    let attn: f32 = 2.0 - shifted.get_attenuation_factor();
    if attn > 0.0 {
        return attn.powi(4) * NoiseEvaluatorType::extrapolate(grid + delta, shifted, perm);
    }

    0.0
}

pub fn floor(x: f32) -> i64 {
    x.floor() as i64
}

pub fn to_f32(x: i64) -> f32 {
    x as f32
}
