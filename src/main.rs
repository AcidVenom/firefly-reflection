#[macro_use]
extern crate glium;
use glium::Surface;

mod snuff;

struct TestState {
    test_mesh: snuff::gfx::Mesh,
    shader_program: snuff::gfx::ShaderProgram,
}

impl TestState {
    fn new(window: &mut snuff::core::Window) -> TestState {
        TestState {
            test_mesh: snuff::gfx::Mesh::create_quad(window.display(), true),
            shader_program: snuff::gfx::ShaderProgram::from_source(
                window.display(), 
                "assets/shaders/simple.vs", 
                "assets/shaders/simple.fs")
                .unwrap()
        }
    }
}

impl snuff::core::GameState for TestState {
    fn on_enter(&mut self) {}

    fn on_leave(&mut self) {}

    fn update(&mut self, dt: f32) {
    }

    fn draw(&mut self, frame: &mut glium::Frame, dt: f32) {
        frame
            .draw(
                self.test_mesh.vertex_buffer(),
                self.test_mesh.index_buffer(),
                self.shader_program.program(),
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let working_dir = args[1].clone();
        assert!(std::env::set_current_dir(&working_dir).is_ok(), format!("[main] Invalid working directory '{}'", working_dir));
        println!("[main] Set working directory to '{}'", working_dir);
    }

    let mut game_loop = snuff::core::GameLoop::new(1280, 720, "Firefly - Reflection", true);

    let window = game_loop.window();
    let test_state = Box::new(TestState::new(window));

    let game_state_manager = game_loop.game_state_manager();
    game_state_manager.add_state("TestState", test_state);
    game_state_manager.switch("TestState");

    game_loop.exec();
}
