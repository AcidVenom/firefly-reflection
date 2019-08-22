use crate::snuff::core;
use crate::snuff::gfx;

pub struct MainState {
    camera: core::Camera,
    fullscreen_quad: gfx::Mesh,
    color_target: gfx::Texture2D,
    background_shader: gfx::ShaderProgram,
    fullscreen_shader: gfx::ShaderProgram
}

impl MainState {
    pub fn new(window: &mut core::Window) -> MainState {
        let display = window.display();

        let mut main_state = MainState {
            camera: core::Camera::new(),
            fullscreen_quad: gfx::Mesh::create_quad(display, false),
            color_target: gfx::Texture2D::empty(display, 1280, 720),
            background_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/fullscreen.vs".to_string(), "assets/shaders/background.fs".to_string()).unwrap(),
            fullscreen_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/fullscreen.vs".to_string(), "assets/shaders/fullscreen.fs".to_string()).unwrap()
        };

        main_state.camera
            .set_orthographic(true)
            .set_orthographic_size_both_f(1280.0, 720.0);

        main_state
    }
}

impl core::GameState for MainState {
    fn on_enter(&mut self) {

    }

    fn update(&mut self, _dt: f32, window: &core::Window) -> Option<String> {

        if window.is_key_released(glium::glutin::VirtualKeyCode::R) {
            return Some(String::from("MenuState"));
        }

        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, _dt: f32) {

        let mut fullscreen_transform = core::Transform::new();

        // Draw background
        let mut target = command_buffer.render_target(vec![&self.color_target]);
        command_buffer.draw_into_target(
            &mut target,
            &mut self.camera,
            &self.fullscreen_quad,
            &mut fullscreen_transform,
            &mut self.background_shader,
            &Vec::new());

        // Final pass
        command_buffer.fullscreen_pass(
            &mut self.camera,
            &mut self.fullscreen_shader, 
            &vec![&self.color_target]);
    }

    fn on_leave(&mut self) {

    }
}