use noise::{NoiseFn, Seedable};
use rand::random_range;

pub struct Random {
}
impl Random {
    pub fn new() -> Self{
        Random {}
    }
}
impl NoiseFn<f64, 2> for Random {
    fn get(&self, point: [f64; 2]) -> f64 {
        random_range(0.0..1.0)
    }
}
impl Seedable for Random {
    fn set_seed(self, seed: u32) -> Self {
        todo!()
    }

    fn seed(&self) -> u32 {
        todo!()
    }
}