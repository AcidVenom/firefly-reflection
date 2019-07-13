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

    fn update(&mut self, dt: f32) {}

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
    let mut window = snuff::core::Window::new(1280, 720, "Firefly - Reflection");
    let mut game_state_manager = snuff::core::GameStateManager::new();

    let test_state = Box::new(TestState::new(&mut window));
    game_state_manager.add_state(String::from("TestState"), test_state);
    game_state_manager.switch(String::from("TestState"));

    let mut frame_count = 0;
    while window.process_events() {
        game_state_manager.update(0.0);

        let mut target = window.begin_frame();

        game_state_manager.draw(&mut target, 0.0);

        window.end_frame(target);

        frame_count += 1;
    }
}
