pub mod scene;

use std::io;
use std::ops::Index;
use crate::map::{Map, MapType};
use crate::{SEC_AS_NANOSEC, TARGET_FPS};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use crossterm::event::{poll, read, Event, KeyCode};
use crate::map::MapType::{Circle, Square};
use crate::terminal::scene::{Scene, SceneType};

pub struct Terminal {
    pub map: Map,
    pub fps: u32,
    pub args: Vec<String>,
    pub dimensions: (u16, u16),
    pub scenes: Vec<Scene>,
    pub current_scene: usize,
}

impl Terminal {
    pub fn new(args: Vec<String>) -> Self {
        let mut terminal = Terminal {
            map: set_map_from_args(&args),
            fps: 0,
            args,
            dimensions: size().unwrap(),
            scenes: Vec::new(),
            current_scene: 0,
        };
        terminal.map.calculate_size(&terminal.dimensions);
        terminal.map.generate_map();
        terminal
        
    }
    pub fn set_scenes(&mut self, scenes: Vec<Scene>) {
        self.scenes = scenes;
    }
    pub fn refresh_scene(&mut self) {
        let scene = &mut self.scenes[self.current_scene];
        match scene.scene_type {
            SceneType::Settings => {
                disable_raw_mode().unwrap();
                println!("\x1B[3J\x1Bc\x1B[?25l{}", scene.get_printable_content());
                enable_raw_mode().unwrap();
            }
            SceneType::Game => {
                disable_raw_mode().unwrap();
                print!("\x1Bc");
                print!("\x1B[?25l");
                print!("FPS: {}\n", self.fps);

                println!("{}", self.map.as_string());
                enable_raw_mode().unwrap();
            }
        }
    }

    pub fn refresh_fps(&self) {
        disable_raw_mode().unwrap();
        print!("\x1B[H\x1B[K");
        print!("FPS: {}\n", self.fps);
        enable_raw_mode().unwrap();
    }
    pub fn poll_keys(&mut self) -> bool{
        let scene = &mut self.scenes[self.current_scene];
        loop {
            //TODO: decide how long should the polling last
            if poll(Duration::from_micros(500)).unwrap() {
                let event = read().unwrap();
                return match event {
                    Event::Resize(width, height) => {
                        self.dimensions = (width, height);
                        self.map.calculate_size(&self.dimensions);
                        if self.map.is_simplex() { self.map.generators.set_simplex_seed(); }
                        true
                    },
                    Event::Key(_) => {
                        return match scene.scene_type {
                            SceneType::Settings => {
                                match event.as_key_event().unwrap().code {
                                    KeyCode::Esc => false,
                                    KeyCode::Enter => {
                                        let scene_id = scene.buttons[scene.current_button_id].next_scene_id;
                                        if scene_id < 0 { false } else {
                                        self.current_scene = scene_id as usize; //TODO: change through the scene_selector in Scene
                                        true
                                        }
                                    }

                                    KeyCode::Up => {
                                        scene.deselect_button(scene.buttons[scene.current_button_id].content_id);
                                        scene.button_up();
                                        scene.select_button(scene.buttons[scene.current_button_id].content_id);
                                        true
                                    },
                                    KeyCode::Down => {
                                        scene.deselect_button(scene.buttons[scene.current_button_id].content_id);
                                        scene.button_down();
                                        scene.select_button(scene.buttons[scene.current_button_id].content_id);
                                        true
                                    },

                                    _ => false,
                                }
                            }
                            SceneType::Game => {
                                match event.as_key_event().unwrap().code {
                                    KeyCode::Esc => false,
                                    KeyCode::Up => {
                                        self.map.pos_offset.1 += 1;
                                        true
                                    },
                                    KeyCode::Down => {
                                        self.map.pos_offset.1 -= 1;
                                        true
                                    },
                                    KeyCode::Right => {
                                        self.map.pos_offset.0 += 1;
                                        true
                                    },
                                    KeyCode::Left => {
                                        self.map.pos_offset.0 -= 1;
                                        true
                                    },
                                    KeyCode::Char(' ') => {
                                        if self.map.is_simplex() {
                                            self.map.generators.set_simplex_seed();
                                        }
                                        true
                                    },
                                    KeyCode::Char('s') => {
                                        self.map.map_type = Square;
                                        true
                                    },
                                    KeyCode::Char('c') => {
                                        self.map.map_type = Circle;
                                        true
                                    },
                                    _ => false,
                                }
                            }
                        }
                    },
                    //TODO: instead of true something else to not regenerate the map every event 
                    _ => true,
                    /*
                        Event::Mouse(event) => println!("{:?}", event),
                        #[cfg(feature = "bracketed-paste")]
                        Event::Paste(data) => println!("{:?}", data),
                        Event::FocusGained => println!("FocusGained"),
                        Event::FocusLost => println!("FocusLost"),
                     */
                }
            }
        }
    }
    pub fn exit() -> io::Result<()> {
        disable_raw_mode()?;
        print!("\x1Bc");
        Ok(())
    }
}

pub struct MainLoop {
    pub start_time: SystemTime,
    pub loop_counter: u32,
}
impl MainLoop {
    pub fn new() -> Self {
        MainLoop {
            start_time: SystemTime::now(),
            loop_counter: 0,
        }
    }
    
    pub fn reset_loop(&mut self) {
        self.start_time = SystemTime::now();
        self.loop_counter = 0;
    }
    
    pub fn head(&mut self, terminal: &mut Terminal) {
        if self.start_time.elapsed().unwrap().as_secs() == 1 {
            terminal.fps = self.loop_counter;
            self.reset_loop();
            // TODO: terminal.refresh_fps();
        }
    }
    
    pub fn tail(&mut self) {
        self.loop_counter += 1;
        let difference: i128 = (SEC_AS_NANOSEC / TARGET_FPS * self.loop_counter) as i128
            - self.start_time.elapsed().unwrap().as_nanos() as i128;
        if difference > 0 {
            sleep(Duration::new(0, difference as u32));
        }
    }
}

pub fn set_map_from_args(args: &Vec<String>) -> Map {
    let mut map_type = Square;
    if args.len() > 1 {
        if args.len() > 2 { 
            match args[2].as_str() { 
                "square" => map_type = Square,
                "circle" => map_type = Circle,
                _ => panic!("Could find given map type [2. argument]")
            }
        }
        match args[1].as_str() {
            "random" => Map::new(0, map_type),
            "simplex" => Map::new(1, map_type),
            "wfc" => Map::new(2, map_type),
            _ => panic!("Could not find given generator [1. argument]")
        }
    } else {
        Map::new(0, map_type)
    }
}