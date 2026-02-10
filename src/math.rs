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
