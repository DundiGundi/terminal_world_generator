use noise::{NoiseFn, Seedable};

//TODO: WFC algorithm
//Wave Function Collapse
pub struct WFC {
}
impl WFC {
    pub fn new() -> Self{
        WFC {}
    }
}
impl NoiseFn<f64, 2> for WFC {
    fn get(&self, point: [f64; 2]) -> f64 {
        0.0
    }
}
impl Seedable for WFC {
    fn set_seed(self, seed: u32) -> Self {
        todo!()
    }

    fn seed(&self) -> u32 {
        todo!()
    }
}