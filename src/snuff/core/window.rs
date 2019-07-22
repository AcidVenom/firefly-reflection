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

#[derive(PartialEq, Clone, Copy)]
pub enum MouseState {
    Pressed,
    Released,
    Down,
    Up,
    Moved,
    Scroll
}

pub struct MouseEvent {
    button: glutin::MouseButton,
    state: MouseState,
    screen_pos: nalgebra_glm::Vec2,
    scroll: f32
}

pub struct Window {
    client_width: u16,
    client_height: u16,
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    default_texture: snuff::gfx::Texture2D,
    fullscreen_quad: snuff::gfx::Mesh,
    key_states: std::collections::HashMap<glutin::VirtualKeyCode, KeyState>,
    mouse_states: std::collections::HashMap<glutin::MouseButton, MouseState>,
    old_mouse_pos: nalgebra_glm::Vec2,
    current_mouse_pos: nalgebra_glm::Vec2,
    mouse_scroll: f32
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
            mouse_states: std::collections::HashMap::new(),
            old_mouse_pos: nalgebra_glm::vec2(0.0, 0.0),
            current_mouse_pos: nalgebra_glm::vec2(0.0, 0.0),
            mouse_scroll: 0.0
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
    fn handle_mouse_events(&mut self, events: Vec<MouseEvent>) {
        for evt in events.iter() {
            match evt.state {
                MouseState::Moved => {
                    self.old_mouse_pos = self.current_mouse_pos;
                    self.current_mouse_pos = evt.screen_pos;
                },
                MouseState::Scroll => {
                    self.mouse_scroll = evt.scroll;
                },
                MouseState::Pressed | MouseState::Released => {
                    let button = &evt.button;
                    let current_pressed = evt.state == MouseState::Pressed;
                    let mut new_state = MouseState::Down;

                    match self.mouse_states.get(button) {
                        Some(state) => {
                            let prev_state = *state;

                            if (prev_state == MouseState::Up || prev_state == MouseState::Released)
                                && current_pressed
                            {
                                new_state = MouseState::Pressed;
                            }

                            if (prev_state == MouseState::Down || prev_state == MouseState::Pressed)
                                && !current_pressed
                            {
                                new_state = MouseState::Released;
                            }
                        }
                        None => {
                            new_state = if current_pressed {
                                MouseState::Pressed
                            } else {
                                MouseState::Released
                            };
                        }
                    }

                    self.mouse_states.insert(*button, new_state);
                },
                _ => {}
            }
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn reset_input_states(&mut self) {
        for it in self.key_states.iter_mut() {
            let old_state = *it.1;
            *it.1 = match old_state {
                KeyState::Pressed => KeyState::Down,
                KeyState::Released => KeyState::Up,
                _ => old_state,
            };
        }

        for it in self.mouse_states.iter_mut() {
            let old_state = *it.1;
            *it.1 = match old_state {
                MouseState::Pressed => MouseState::Down,
                MouseState::Released => MouseState::Up,
                _ => old_state,
            };
        }

        self.mouse_scroll = 0.0;
        self.old_mouse_pos = self.current_mouse_pos;
    }

    //---------------------------------------------------------------------------------------------------
    pub fn process_events(&mut self) -> bool {
        let mut closed = false;
        let mut key_events: Vec<glium::glutin::KeyboardInput> = Vec::new();
        let mut mouse_events: Vec<MouseEvent> = Vec::new();
        let old_mouse_pos = self.old_mouse_pos;

        self.reset_input_states();

        self.events_loop.poll_events(|evt| match evt {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                //---------------------------------------------------------------------------------------------------
                glutin::WindowEvent::KeyboardInput {
                    input,
                    ..
                } => key_events.push(input),
                //---------------------------------------------------------------------------------------------------
                glutin::WindowEvent::MouseInput {
                    state,
                    button,
                    ..
                } => mouse_events.push(MouseEvent{ 
                    button, 
                    state: if state == glutin::ElementState::Pressed { MouseState::Pressed } else { MouseState::Released }, 
                    screen_pos: old_mouse_pos, 
                    scroll: 0.0 
                }),
                //---------------------------------------------------------------------------------------------------
                glutin::WindowEvent::MouseWheel {
                    delta,
                    ..
                } => mouse_events.push(MouseEvent{ 
                    button: glutin::MouseButton::Left, 
                    state: MouseState::Scroll, 
                    screen_pos: old_mouse_pos, 
                    scroll: match delta { 
                        glutin::MouseScrollDelta::PixelDelta(d) => { d.y as f32 },
                        glutin::MouseScrollDelta::LineDelta(_, dy) => { dy }
                }}),
                //---------------------------------------------------------------------------------------------------
                glutin::WindowEvent::CursorMoved {
                    position,
                    ..
                } => mouse_events.push(MouseEvent{
                    button: glutin::MouseButton::Left,
                    state: MouseState::Moved,
                    screen_pos: nalgebra_glm::vec2(position.x as f32, position.y as f32),
                    scroll: 0.0
                }),
                //---------------------------------------------------------------------------------------------------
                _ => (),
            },
            _ => (),
        });

        self.handle_key_events(key_events);
        self.handle_mouse_events(mouse_events);

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
    pub fn is_mouse_button_pressed(&self, button: glutin::MouseButton) -> bool {
        match self.mouse_states.get(&button) {
            Some(state) => *state == MouseState::Pressed,
            None => false,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn is_mouse_button_released(&self, button: glutin::MouseButton) -> bool {
        match self.mouse_states.get(&button) {
            Some(state) => *state == MouseState::Released,
            None => false,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn is_mouse_button_down(&self, button: glutin::MouseButton) -> bool {
        match self.mouse_states.get(&button) {
            Some(state) => *state == MouseState::Down || *state == MouseState::Pressed,
            None => false,
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn absolute_to_clip_space(&self, p : nalgebra_glm::Vec2) -> nalgebra_glm::Vec2 {
        let mut clip_space = nalgebra_glm::vec2(p.x / self.client_width as f32, 1.0 - (p.y / self.client_height as f32));
        clip_space *= 2.0;
        clip_space - nalgebra_glm::vec2(1.0, 1.0)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn mouse_position(&self) -> nalgebra_glm::Vec2 {
        self.current_mouse_pos
    }

    //---------------------------------------------------------------------------------------------------
    pub fn mouse_relative_position(&self) -> nalgebra_glm::Vec2 {
        self.absolute_to_clip_space(self.current_mouse_pos)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn mouse_delta(&self) -> nalgebra_glm::Vec2 {
        self.old_mouse_pos - self.current_mouse_pos
    }

    //---------------------------------------------------------------------------------------------------
    pub fn mouse_relative_delta(&self) -> nalgebra_glm::Vec2 {
        self.absolute_to_clip_space(self.mouse_delta())
    }

    //---------------------------------------------------------------------------------------------------
    pub fn mouse_scroll(&self) -> f32 {
        self.mouse_scroll
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
