use bevy::math::Vec2;
use rand::{
    distr::uniform::{SampleRange, SampleUniform},
    *,
};

pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::rng().random_range(range)
}

pub fn random_chance(p: f32) -> bool {
    rand::rng().random_bool(p.into())
}

pub fn random_vec(rmin: f32, rmax: f32) -> Vec2 {
    let r = rand::rng().random_range(rmin..rmax);
    let a = rand::rng().random_range(0.0..std::f32::consts::PI * 2.0);

    let x = r * a.cos();
    let y = r * a.sin();

    (x, y).into()
}
