#[macro_use]
extern crate glium;
use glium::Surface;

mod snuff;

struct TestState {
    test_mesh: snuff::gfx::Mesh,
    shader_program: glium::Program,
}

impl TestState {
    fn new(window: &mut snuff::core::Window) -> TestState {
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 1.0, 1.0, 1.0);
            }
        "#;

        TestState {
            test_mesh: snuff::gfx::Mesh::create_quad(window.display(), true),
            shader_program: glium::Program::from_source(
                window.display(),
                vertex_shader_src,
                fragment_shader_src,
                None,
            )
            .unwrap(),
        }
    }
}

impl snuff::core::GameState for TestState {
    fn on_enter(&mut self) {}

    fn on_leave(&mut self) {}

    fn update(&mut self, dt: f32) {
        println!("{}", dt);
    }

    fn draw(&mut self, frame: &mut glium::Frame, dt: f32) {
        frame
            .draw(
                &self.test_mesh.vertex_buffer,
                &self.test_mesh.index_buffer,
                &self.shader_program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

fn main() {
    let mut game_loop = snuff::core::GameLoop::new(1280, 720, "Firefly - Reflection", true);

    let window = game_loop.window();
    let test_state = Box::new(TestState::new(window));

    let game_state_manager = game_loop.game_state_manager();
    game_state_manager.add_state(String::from("TestState"), test_state);
    game_state_manager.switch(String::from("TestState"));

    game_loop.exec();
}
