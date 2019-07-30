use crate::snuff::core;
use crate::snuff::gfx;

pub struct MenuState {

}

impl MenuState {
    pub fn new(window : &core::Window) -> MenuState {
        MenuState{}
    }
}

impl core::GameState for MenuState {
    fn on_enter(&mut self) {

    }

    fn update(&mut self, dt: f32, window: &core::Window) -> Option<String> {
        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, dt: f32) {

    }

    fn on_leave(&mut self) {

    }
}