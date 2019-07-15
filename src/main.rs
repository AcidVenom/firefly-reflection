#[macro_use]
extern crate glium;
extern crate time;

use glium::Surface;

mod snuff;

struct TestState {
    test_mesh: snuff::gfx::Mesh,
    shader_program: snuff::gfx::ShaderProgram,
    angle: f32
}

impl TestState {
    fn new(window: &mut snuff::core::Window) -> TestState {
        TestState {
            test_mesh: snuff::gfx::Mesh::create_quad(window.display(), true),
            shader_program: snuff::gfx::ShaderProgram::from_source(
                window.display(),
                "assets/shaders/simple.vs",
                "assets/shaders/simple.fs",
            )
            .unwrap(),
            angle: 0.0
        }
    }
}

impl snuff::core::GameState for TestState {
    fn on_enter(&mut self) {}

    fn on_leave(&mut self) {}

    fn update(&mut self, dt: f32) {}

    fn draw(&mut self, frame: &mut glium::Frame, dt: f32) {
        self.angle += dt * 3.14159;

        let aspect = 720.0 / 1280.0;
        let ortho_size = 5.0;
        let half_size = ortho_size * 0.5;

        let rot = nalgebra_glm::rotate(&nalgebra_glm::identity(), self.angle, &nalgebra_glm::vec3(0.0, 0.0, 1.0));
        let m = rot * nalgebra_glm::ortho_lh(-half_size, half_size, -half_size * aspect, half_size * aspect, 0.01, 100.0);
        
        let r1 = m.row(0);
        let r2 = m.row(1);
        let r3 = m.row(2);
        let r4 = m.row(3);

        let uniforms = uniform!
        {
            matrix: [
                [ r1[0] as f32, r1[1] as f32, r1[2] as f32, r1[3] as f32 ],
                [ r2[0] as f32, r2[1] as f32, r2[2] as f32, r2[3] as f32 ],
                [ r3[0] as f32, r3[1] as f32, r3[2] as f32, r3[3] as f32 ],
                [ r4[0] as f32, r4[1] as f32, r4[2] as f32, r4[3] as f32 ]
            ]
        };

        frame
            .draw(
                self.test_mesh.vertex_buffer(),
                self.test_mesh.index_buffer(),
                self.shader_program.program(),
                &uniforms,
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
    game_state_manager.add_state("TestState", test_state);
    game_state_manager.switch("TestState");

    game_loop.exec();
}
