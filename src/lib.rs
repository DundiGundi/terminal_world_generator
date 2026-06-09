mod map;
mod terminal;

use crate::terminal::scene::Scene;
use crate::terminal::scene::SceneType::{Game, Settings};
use crate::terminal::{MainLoop, Terminal};
use std::{env, io};

const SEC_AS_NANOSEC: u32 = 1000000000;
const TARGET_FPS: u32 = 60;

pub fn run() -> io::Result<()> {
    let main_menu_string = String::from("\
╭────────────────────╮
│ World-Gen Reloaded │
│                    │
│    Version: 0.1    │
╰────────────────────╯
#Start#
#Settings#
#Quit#");
    let mut main_menu = Scene::new(main_menu_string, Settings, vec![2,1,-1]);

    let mut terminal = Terminal::new(env::args().collect());
    main_menu.center_content(terminal.dimensions);
    // TODO: buttons for all scene
    terminal.set_scenes(vec![main_menu, Scene::new("".to_string(), Settings, vec![0]), Scene::new("".to_string(), Game, vec![0])]);

    terminal.refresh_scene();
    //TODO: fps is broken a polling miatt
    match terminal.poll_keys() {
        false => return Terminal::exit(),
        true => {terminal.refresh_scene()},
    }

    let mut main_loop = MainLoop::new();
    //main loop
    loop {
        // HEAD
        main_loop.head(&mut terminal);
        
        // ##################################################
        // BODY
        match terminal.poll_keys() {
            false => break,
            true => {
                terminal.map.generate_map();
                terminal.refresh_scene();
            },
        }
        // ##################################################
        // TAIL
        main_loop.tail();
    }
    Terminal::exit()
}