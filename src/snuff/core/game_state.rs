use crate::snuff;
use std::collections::HashMap;

pub trait GameState {
    fn on_enter(&mut self);
    fn on_leave(&mut self);
    fn update(&mut self, dt: f32, window: &snuff::core::Window) -> Option<String>;
    fn draw(&mut self, command_buffer: &mut snuff::gfx::CommandBuffer, dt: f32);
}

pub struct GameStateManager {
    states: HashMap<String, Box<GameState>>,
    current_state: String,
}

impl GameStateManager {
    //---------------------------------------------------------------------------------------------------
    pub fn new() -> GameStateManager {
        GameStateManager {
            states: HashMap::new(),
            current_state: String::new(),
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn add_state<'a>(&mut self, name: &'a str, state: Box<GameState>) {
        if self.states.contains_key(name) {
            println!("[GameStateManager] Attempted to add a state with key '{}', but it already exists, skipping", name);
            return;
        }

        self.states.insert(String::from(name), state);
    }

    //---------------------------------------------------------------------------------------------------
    pub fn switch<'a>(&mut self, name: &'a str) {
        if let Some(s) = self.get_current_state() {
            s.on_leave();
        }

        match self.states.get_mut(name) {
            Some(s) => {
                s.on_enter();
                self.current_state = String::from(name);
                println!("[GameStateManager] Switched to state '{}'", name);
            }

            None => {
                println!("[GameStateManager] Attempted to switch to state '{}', but it doesn't exist, staying in current state", name);
            }
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn get_current_state(&mut self) -> Option<&mut GameState> {
        if !self.current_state.is_empty() {
            match self.states.get_mut(&self.current_state) {
                Some(s) => Some(s.as_mut()),
                None => None,
            }
        } else {
            None
        }
    }
}
