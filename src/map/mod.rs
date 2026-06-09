mod generators;

use crate::map::MapType::{Circle, Square};
use noise::{NoiseFn, Seedable, Simplex};
use rand::random_range;
use std::cmp::Ordering;
use std::ops::Mul;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::map::generators::Generators;
use crate::map::generators::random::Random;
use crate::map::generators::wfc::WFC;

pub struct Map {
    pub size: u16,
    pub content: Vec<Color>,
    pub average: Color,
    pub generator_id: u8,
    pub generators: Generators,
    pub map_type: MapType,
    pub pos_offset: (i32, i32),
}

impl Map {
    pub fn new(generator_id: u8, map_type: MapType) -> Map {
        Map {
            size: 0,
            content: Vec::new(),
            average: Color::black(),
            generator_id,
            generators: Generators(Random{},
                                   Simplex::new(random_range(0..SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()) as u32),
                                   WFC{}),
            map_type,
            pos_offset: (0, 0),
        }
    }
    pub fn generate_map(&mut self) {
        // color the tiles
        self.content.clear();
        match self.map_type {
            Square => {
                for i in 0..(self.size * self.size) {
                    // TODO: make this and that in the circle part a bit beautifuler
                    let mut color = self.generators.get_color_from_generator(self.generator_id, &Vec2((i % self.size) as i32 + self.pos_offset.0, (self.size - i / self.size) as i32 + self.pos_offset.1));
                    if self.is_random() {
                        color.0 = (color.0 as f64 * random_range(0.0..1.0)) as u8;
                        color.1 = (color.1 as f64 * random_range(0.0..1.0)) as u8;
                        color.2 = (color.2 as f64 * random_range(0.0..1.0)) as u8;
                        self.content.push(color.copy());
                    } else {
                        self.content.push(color);

                    }
                }
                self.average = self.average_colors(&self.content);
            },
            Circle => {
                let mut colored_tiles: Vec<Color> = Vec::new();
                let origin = Vec2(self.size as i32 / 2, self.size as i32 / 2);
                for i in 0..(self.size * self.size) {
                    let point = Vec2((i % self.size) as i32 + self.pos_offset.0, (self.size - i / self.size) as i32 + self.pos_offset.1);
                    if origin.distance_to(&point) <= self.size as f32 / 2f32 {
                        //let randoms: [u8 ; 3] = [random_range(0..255), random_range(0..255), random_range(0..255)];

                        let mut color = self.generators.get_color_from_generator(self.generator_id, &point);
                        if self.is_random() {
                            color.0 = (color.0 as f64 * random_range(0.0..1.0)) as u8;
                            color.1 = (color.1 as f64 * random_range(0.0..1.0)) as u8;
                            color.2 = (color.2 as f64 * random_range(0.0..1.0)) as u8;
                            self.content.push(color.copy());
                            colored_tiles.push(color);
                        } else {
                            self.content.push(color.copy());
                            colored_tiles.push(color);
                        }

                    } else {
                        self.content.push(Color(0, 0, 0));
                    }

                }
                self.average = self.average_colors(&colored_tiles);
            }
        }
    }

    // TODO: colors.len can be 0 when no circle can be seen
    pub fn average_colors(&self, colors: &Vec<Color>) -> Color {
        let mut color: [u32; 3] = [0, 0, 0];
        let size = colors.len() as u32;
        for val in colors.iter() {
            color[0] += val.0 as u32;
            color[1] += val.1 as u32;
            color[2] += val.2 as u32;
        }
        Color(
            (color[0] / size) as u8,
            (color[1] / size) as u8,
            (color[2] / size) as u8,
        )
    }

    pub fn calculate_size(&mut self, terminal_size: &(u16, u16)) {
        let width = terminal_size.0 / 3 - 2;
        let height = terminal_size.1 - 2 - 2; // padding - fps counter
        match width.cmp(&height) {
            /*Ordering::Less => println!("Recommended maximum world size: {width} * {width}"),
            Ordering::Greater => println!("Recommended maximum world size: {height} * {height}"),
            Ordering::Equal => println!("Recommended maximum world size: {width} * {width}"),*/
            Ordering::Less => self.size = width,
            Ordering::Greater => self.size = height,
            Ordering::Equal => self.size = width,
        }
    }

    pub fn is_random(&self) -> bool {
        self.generator_id == 0
    }
    pub fn is_simplex(&self) -> bool {
        self.generator_id == 1

    }
    pub fn is_wfc(&self) -> bool {
        self.generator_id == 2
    }
    
    pub fn as_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.size {
            s.push_str("\x1B[0m\n   ");
            for j in 0..self.size {
                let color = &self.content[(i * self.size + j) as usize];
                s.push_str(format!("\x1B[48;2;{};{};{}m   ", color.0, color.1, color.2).as_str());
            }
        }
        s.push_str(
            format!("\x1B[0m   \x1B[48;2;{};{};{}m   \x1B[0m\n", self.average.0, self.average.1, self.average.2, ).as_str(),
        );
        s
    }
}

pub enum MapType {
    Square,
    Circle
}

pub struct Color(pub u8, pub u8, pub u8);
impl Color {
    pub fn black() -> Self {
        Color(0, 0, 0)
    }
    pub fn copy(&self) -> Self { Color(self.0, self.1, self.2)}
}
pub struct Vec2(pub i32, pub i32);
impl Vec2 {
    pub fn distance_to(&self, other_vec: &Vec2) -> f32 {
        (((other_vec.0 - self.0).pow(2) + (other_vec.1 - self.1).pow(2))as f32).sqrt()
    }
}