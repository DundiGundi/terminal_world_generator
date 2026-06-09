use std::time::{SystemTime, UNIX_EPOCH};
use noise::{NoiseFn, Seedable, Simplex};
use rand::random_range;
use crate::map::{Color, Vec2};
use crate::map::generators::random::Random;
use crate::map::generators::wfc::WFC;

pub mod random;
pub mod wfc;

pub struct Generators(
    pub Random,
    pub Simplex,
    pub WFC,
);
impl Generators {
    pub fn get_color_from_generator(&self, generator: u8, point: &Vec2) -> Color {
        match generator {
            0 => {
                let multiplier = self.0.get([point.0 as f64, point.1 as f64]);
                Color((255f64 * multiplier) as u8, (255f64 * multiplier) as u8, (255f64 * multiplier) as u8)
            },
            1 => {
                let multiplier = self.1.get([point.0 as f64, point.1 as f64]);
                Color((255f64 * multiplier) as u8, (255f64 * multiplier) as u8, (255f64 * multiplier) as u8)
            },
            2 => {
                let multiplier = self.2.get([point.0 as f64, point.1 as f64]);
                Color((255f64 * multiplier) as u8, (255f64 * multiplier) as u8, (255f64 * multiplier) as u8)
            },
            _ => panic!("There is no such generator [get_color_from_generator]")
        }
    }

    pub fn set_simplex_seed(&mut self) {
        self.1 = self.1.set_seed(random_range(0..SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()) as u32);
    }
}