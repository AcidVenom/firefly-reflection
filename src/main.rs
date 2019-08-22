#![allow(dead_code)]
#[macro_use]

extern crate glium;
extern crate image;
extern crate time;
extern crate notify;
extern crate rodio;

mod snuff;
mod firefly;

fn main() {
    let mut game_loop = snuff::core::GameLoop::new(1280, 720, "Firefly - Reflection", true);

    let window = game_loop.window();

    let menu_state = Box::new(firefly::MenuState::new(window));
    let main_state = Box::new(firefly::MainState::new(window));
    
    let game_state_manager = game_loop.game_state_manager();

    game_state_manager.add_state("MenuState", menu_state);
    game_state_manager.add_state("MainState", main_state);

    game_state_manager.switch("MenuState");

    game_loop.exec();
}
