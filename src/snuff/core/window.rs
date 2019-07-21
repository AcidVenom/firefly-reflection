use crate::snuff;
use crate::snuff::gfx::CommandBuffer;

use glium::glutin;

#[derive(PartialEq, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
    Down,
    Up,
}

pub struct Window {
    client_width: u16,
    client_height: u16,
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    default_texture: snuff::gfx::Texture2D,
    fullscreen_quad: snuff::gfx::Mesh,
    key_states: std::collections::HashMap<glutin::VirtualKeyCode, KeyState>,
}

impl Window {
    //---------------------------------------------------------------------------------------------------
    pub fn new(width: u16, height: u16, title: &'static str, vsync: bool) -> Window {
        let events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_dimensions((u32::from(width), u32::from(height)).into())
            .with_title(title);

        let cb = glutin::ContextBuilder::new().with_vsync(vsync);
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        let default_texture =
            snuff::gfx::Texture2D::from_data(&display, &vec![255, 255, 255, 255], 1, 1);
        let fullscreen_quad = snuff::gfx::Mesh::create_quad(&display, false);

        Window {
            client_width: width,
            client_height: height,
            events_loop,
            display,
            default_texture,
            fullscreen_quad,
            key_states: std::collections::HashMap::new(),
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn display(&mut self) -> &mut glium::Display {
        &mut self.display
    }

    //---------------------------------------------------------------------------------------------------
    pub fn client_width(&self) -> u16 {
        self.client_width
    }

    //---------------------------------------------------------------------------------------------------
    pub fn client_height(&self) -> u16 {
        self.client_height
    }

    //---------------------------------------------------------------------------------------------------
    fn handle_key_events(&mut self, events: Vec<glium::glutin::KeyboardInput>) {
        for evt in events.iter() {
            let keycode = &evt.virtual_keycode.unwrap();
            let current_pressed = evt.state == glutin::ElementState::Pressed;
            let mut new_state = KeyState::Down;

            match self.key_states.get(keycode) {
                Some(state) => {
                    let prev_state = *state;

                    if (prev_state == KeyState::Up || prev_state == KeyState::Released)
                        && current_pressed
                    {
                        new_state = KeyState::Pressed;
                    }

                    if (prev_state == KeyState::Down || prev_state == KeyState::Pressed)
                        && !current_pressed
                    {
                        new_state = KeyState::Released;
                    }
                }
                None => {
                    new_state = if current_pressed {
                        KeyState::Pressed
                    } else {
                        KeyState::Released
                    };
                }
            }

            self.key_states.insert(*keycode, new_state);
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn reset_key_states(&mut self) {
        for it in self.key_states.iter_mut() {
            let old_state = *it.1;
            *it.1 = match old_state {
                KeyState::Pressed => KeyState::Down,
                KeyState::Released => KeyState::Up,
                _ => old_state,
            };
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn process_events(&mut self) -> bool {
        let mut closed = false;
        let mut key_events: Vec<glium::glutin::KeyboardInput> = Vec::new();

        self.reset_key_states();

        self.events_loop.poll_events(|evt| match evt {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::KeyboardInput {
                    input,
                    ..
                } => key_events.push(input),
                _ => (),
            },
            _ => (),
        });

        self.handle_key_events(key_events);

        !closed
    }

    //---------------------------------------------------------------------------------------------------
    pub fn is_key_pressed(&self, key: glutin::VirtualKeyCode) -> bool {
        match self.key_states.get(&key) {
            Some(state) => *state == KeyState::Pressed,
            None => false,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn is_key_released(&self, key: glutin::VirtualKeyCode) -> bool {
        match self.key_states.get(&key) {
            Some(state) => *state == KeyState::Released,
            None => false,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn is_key_down(&self, key: glutin::VirtualKeyCode) -> bool {
        match self.key_states.get(&key) {
            Some(state) => *state == KeyState::Down || *state == KeyState::Pressed,
            None => false,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn begin_frame(&mut self, time: f32) -> CommandBuffer {
        snuff::gfx::CommandBuffer::new(
            &self.display,
            &self.default_texture,
            &self.fullscreen_quad,
            time,
        )
    }
}
