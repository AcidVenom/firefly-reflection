use crate::snuff::core;
use crate::snuff::gfx;
use crate::firefly::objects;
use crate::firefly::objects::GameObject;

#[derive(PartialEq)]
enum FadeMode {
    In,
    Out,
    Done
}

pub struct MainState {
    camera: core::Camera,
    fullscreen_quad: gfx::Mesh,
    quad: gfx::Mesh,
    color_target: gfx::Texture2D,
    background_shader: gfx::ShaderProgram,
    fullscreen_shader: gfx::ShaderProgram,
    shader: gfx::ShaderProgram,
    fade: f32,
    fade_timer: f32,
    fade_from: f32,
    fade_mode: FadeMode,
    player: objects::Player
}

impl MainState {
    pub fn new(window: &mut core::Window) -> MainState {
        let display = window.display();

        let mut main_state = MainState {
            camera: core::Camera::new(),
            fullscreen_quad: gfx::Mesh::create_quad(display, false),
            quad: gfx::Mesh::create_quad(display, true),
            color_target: gfx::Texture2D::empty(display, 1280, 720),
            background_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/fullscreen.vs".to_string(), "assets/shaders/background.fs".to_string()).unwrap(),
            fullscreen_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/fullscreen.vs".to_string(), "assets/shaders/fullscreen.fs".to_string()).unwrap(),
            shader: gfx::ShaderProgram::from_source(display, "assets/shaders/simple.vs".to_string(), "assets/shaders/simple.fs".to_string()).unwrap(),
            fade: 1.0,
            fade_timer: 0.0,
            fade_from: 1.0,
            fade_mode: FadeMode::In,
            player: objects::Player::new(display)
        };

        main_state.camera
            .set_orthographic(true)
            .set_orthographic_size_both_f(1280.0, 720.0)
            .transform().set_translation_f(0.0, 0.0, -1.0);

        main_state
    }
}

impl MainState {
    fn set_defaults(&mut self) {
        self.fade = 1.0;
        self.fade_in();
    }

    fn fade_in(&mut self) {
        self.fade_mode = FadeMode::In;
        self.fade_from = self.fade;
        self.fade_timer = 0.0;
    }

    fn fade_out(&mut self) {
        self.fade_mode = FadeMode::Out;
        self.fade_from = self.fade;
        self.fade_timer = 0.0;
    }

    fn update_fade(&mut self, dt: f32) {

        if self.fade_mode != FadeMode::Done {
            let fade_duration = 2.0;
            self.fade_timer += dt * (1.0 / fade_duration);
            self.fade_timer = self.fade_timer.min(1.0);
        }

        let ease = core::easing::out_cubic(self.fade_timer);
        
        match &self.fade_mode {
            FadeMode::In => {
                self.fade = self.fade_from * (1.0 - ease);
            },
            FadeMode::Out => {
                self.fade = self.fade_from * (1.0 - ease) + ease;
            },
            FadeMode::Done => {}
        }
    }
}

impl core::GameState for MainState {
    fn on_enter(&mut self) {
        self.set_defaults();
    }

    fn update(&mut self, dt: f32, window: &core::Window) -> Option<String> {

        // Variables
        let camera_damping = 100.0;
        let camera_offset = nalgebra_glm::vec2(0.0, 100.0);

        // Debug

        if window.is_key_released(glium::glutin::VirtualKeyCode::R) {
            return Some(String::from("MenuState"));
        }
        else if window.is_key_released(glium::glutin::VirtualKeyCode::F) {
            self.fade_out();
        }
        else if window.is_key_released(glium::glutin::VirtualKeyCode::G) {
            self.fade_in();
        }

        // Fading

        self.update_fade(dt);
        self.player.update(dt, window);

        // Follow a point around with the camera

        let mut t = self.camera.transform().translation_2d();
        t = nalgebra_glm::lerp(&t, &(self.player.transform().translation_2d() + camera_offset), 1.0 - f32::powf(1.0 / camera_damping, dt));
        t.x = t.x.max(0.0);
        self.camera.transform().set_translation_2d(&t);

        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, _dt: f32) {

        let mut fullscreen_transform = core::Transform::new();

        // Draw background
        let mut target = command_buffer.render_target(vec![&self.color_target]);
        command_buffer.clear(&mut target, 0.0, 0.0, 0.0, 0.0);

        command_buffer.draw_into_target(
            &mut target,
            &mut self.camera,
            &self.fullscreen_quad,
            &mut fullscreen_transform,
            &mut self.background_shader,
            &Vec::new());

        // Render player

        let render_data = self.player.render_data();
        command_buffer.draw_into_target(
            &mut target,
            &mut self.camera,
            &self.quad,
            render_data.transform,
            &mut self.shader,
            &render_data.textures);

        // Final pass
        command_buffer.set_blend_color(1.0, 1.0, 1.0, 1.0 - self.fade);
        command_buffer.fullscreen_pass(
            &mut self.camera,
            &mut self.fullscreen_shader, 
            &vec![&self.color_target]);
    }

    fn on_leave(&mut self) {

    }
}