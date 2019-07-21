#![allow(dead_code)]
#[macro_use]

extern crate glium;
extern crate image;
extern crate time;

mod snuff;

struct TestState {
    test_mesh: snuff::gfx::Mesh,
    test_texture: snuff::gfx::Texture2D,
    test_target_texture: snuff::gfx::Texture2D,
    shader_program: snuff::gfx::ShaderProgram,
    post_process: snuff::gfx::ShaderProgram,
    angle: f32,
    camera: snuff::core::Camera,
}

impl TestState {
    fn new(window: &mut snuff::core::Window) -> TestState {
        TestState {
            test_mesh: snuff::gfx::Mesh::create_quad(window.display(), true),
            test_texture: snuff::gfx::Texture2D::from_data(
                window.display(),
                &vec![
                    0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255,
                ],
                2,
                2,
            )
            .with_nearest_filter(),
            test_target_texture: snuff::gfx::Texture2D::empty(window.display(), 1280, 720),
            shader_program: snuff::gfx::ShaderProgram::from_source(
                window.display(),
                "assets/shaders/simple.vs",
                "assets/shaders/simple.fs",
            )
            .unwrap(),
            post_process: snuff::gfx::ShaderProgram::from_source(
                window.display(),
                "assets/shaders/fullscreen.vs",
                "assets/shaders/fullscreen.fs",
            )
            .unwrap(),
            angle: 0.0,
            camera: snuff::core::Camera::new(),
        }
    }
}

impl snuff::core::GameState for TestState {
    fn on_enter(&mut self) {
        self.camera
            .set_orthographic_size(5.0, 720.0 / 1280.0)
            .transform()
            .set_translation_f(0.0, 0.0, 0.0);
    }

    fn on_leave(&mut self) {}

    fn update(&mut self, _dt: f32, window: &snuff::core::Window) {
        if window.is_key_pressed(glium::glutin::VirtualKeyCode::A) {
            println!("Whoop, pressed 'A' this frame");
        }
    }

    fn draw(&mut self, command_buffer: &mut snuff::gfx::CommandBuffer, dt: f32) {
        self.angle += dt * std::f32::consts::PI;

        let mut transform = snuff::core::Transform::new();
        transform
            .roll(self.angle)
            .set_translation_f(self.angle.sin() * 0.5, 0.0, 1.0);

        let mut target = command_buffer.render_target(vec![&self.test_target_texture]);

        command_buffer.clear(&mut target, 0.1, 0.1, 0.1, 1.0);

        command_buffer.draw_into_target(
            &mut target,
            &mut self.camera,
            &self.test_mesh,
            &mut transform,
            &self.shader_program,
            &vec![&self.test_texture],
        );

        command_buffer.fullscreen_pass(
            &mut self.camera,
            &self.post_process,
            &vec![&self.test_target_texture],
        );
    }
}

fn main() {
    let mut game_loop = snuff::core::GameLoop::new(1280, 720, "Firefly - Reflection", false);

    let window = game_loop.window();
    let test_state = Box::new(TestState::new(window));

    let game_state_manager = game_loop.game_state_manager();
    game_state_manager.add_state("TestState", test_state);
    game_state_manager.switch("TestState");

    game_loop.exec();
}
