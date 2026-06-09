mod button;

use std::env::split_paths;
use std::io;
use std::io::Error;
use std::ops::Not;
use std::time::Duration;
use crossterm::event::{poll, read, KeyCode};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::terminal::scene::button::Button;
use crate::terminal::Terminal;

pub enum SceneType {
    Settings,
    Game
}

pub struct Scene {
    pub content: String,
    pub modified_content: Vec<String>,
    pub scene_type: SceneType,
    // TODO: make the button scrolling 2D not just top to bottom
    pub buttons: Vec<Button>,
    pub current_button_id: usize,
    pub next_scene_id_to_button_id: Vec<isize>
}
impl Scene {
    pub fn new(content: String, scene_type: SceneType, next_scene_id_to_button_id: Vec<isize>) -> Self {
        let mut scene = Scene {
            content,
            modified_content: Vec::new(),
            scene_type,
            //id in terminal vector of scenes
            buttons: Vec::new(),
            current_button_id: 0,
            next_scene_id_to_button_id
        };
        scene.buttons = Scene::find_buttons(&mut scene.content);
        scene
    }
    pub fn center_content(&mut self, dimensions: (u16, u16)) {
        let content_split: Vec<&str> = self.content.split("\n").collect();
        //let c_slice = content_split.split_at(1).1;
        self.modified_content = Vec::new();
        let mut string = String::new();

        for i in 0..content_split.len() {
            if content_split[i].chars().count() >= dimensions.0 as usize { continue }

            let line_left = (dimensions.0 as usize - content_split[i].chars().count()) / 2;
            let line_right = dimensions.0 as usize - line_left - content_split[i].chars().count();
            let mut s = String::new();

            for j in 0..line_left {
                s.push(' ');
            }

            s += content_split[i];
            s = s.replace("\n", "");

            for j in 0..line_right {
                s.push(' ');
            }

            s.push('\n');
            string += s.as_str();
        }

        self.buttons = Scene::find_buttons(&mut string);
        // TODO: pls no manual labour
        // TODO: next scene id kicsit jobban bruh
        for i in 0..self.buttons.len() {
            self.buttons[i].next_scene_id = self.next_scene_id_to_button_id[i];
        }
        let v: Vec<&str> = string.split("#").collect();
        self.modified_content = v.iter().map(|x| x.to_string()).collect();
        if self.buttons.len() > 0 { self.select_button(self.buttons[0].content_id); }
    }

    pub fn select_button(&mut self, button_id: usize) {
        self.modified_content[button_id] = format!("\x1B[47;30m{}\x1B[0m", self.modified_content[button_id]);
    }

    pub fn deselect_button(&mut self, button_id: usize) {
        self.modified_content[button_id] = self.modified_content[button_id].replace("\x1B[47;30m", "");
        self.modified_content[button_id] = self.modified_content[button_id].replace("\x1B[0m", "");
    }
    pub fn button_up(&mut self) {
        if self.current_button_id == 0 {
            self.current_button_id = self.buttons.len() - 1;
        } else {
            self.current_button_id -= 1;
        }
    }

    pub fn button_down(&mut self) {
        if self.current_button_id >= self.buttons.len() - 1 {
            self.current_button_id = 0;
        } else {
            self.current_button_id += 1;
        }
    }

    pub fn get_printable_content(&self) -> String {
        self.modified_content.concat()
    }

    fn find_buttons(string: &String) -> Vec<Button> {
        let split: Vec<&str> = string.split("#").collect();
        let mut buttons: Vec<Button> = Vec::new();
        for i in 0..split.len() {
            // every second string is a button
            if i % 2 == 1 { buttons.push(Button::new(i, 0)) }
        }
        buttons
    }
    /* old implementation
    // TODO: after every content modification!!!!
    fn find_buttons(string: &mut String) -> Vec<Button> {
    // button definition: #button#
        let split: Vec<&str> = string.split("#").collect();
        //without the #
        let mut new_string = String::new();
        let mut buttons: Vec<Button> = Vec::new();
        let mut toggle = false;
        // toggle when moving from the left border to the right border or vice versa
        let mut button = Button::new((0, 0), 0);
        for i in 0..split.len() {
            
            if !toggle {
                new_string += split[i];
                button.borders.0 = new_string.chars().count();
            } else {
                new_string += split[i];
                button.borders.1 = new_string.chars().count() - 1;
                buttons.push(button.clone());
            }
            toggle = !toggle;
        }
        // *string = new_string; not needed, # removal handled after find_button
        buttons
    }
    */
}