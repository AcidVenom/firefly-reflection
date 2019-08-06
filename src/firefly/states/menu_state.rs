use crate::snuff::core;
use crate::snuff::gfx;

pub struct MenuState {
    camera: core::Camera,
    quad: gfx::Mesh,
    menu_shader: gfx::ShaderProgram,
    title_texture: gfx::Texture2D,
    subtitle_texture: gfx::Texture2D,
    ground_texture: gfx::Texture2D
}

impl MenuState {
    pub fn new(window: &mut core::Window) -> MenuState {
        let display = window.display();

        let mut menu_state = MenuState {
            camera: core::Camera::new(),
            quad: gfx::Mesh::create_quad(display, true),
            menu_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/widget.vs".to_string(), "assets/shaders/widget.fs".to_string()).unwrap(),
            title_texture: gfx::Texture2D::from_image(display, "assets/textures/menu/title.png"),
            subtitle_texture: gfx::Texture2D::from_image(display, "assets/textures/menu/subtitle.png"),
            ground_texture: gfx::Texture2D::from_image(display, "assets/textures/menu/ground.png")
        };

        menu_state.camera.set_orthographic(true);
        menu_state.camera.set_orthographic_size(1280.0, 720.0 / 1280.0);

        menu_state
    }
}

impl core::GameState for MenuState {
    fn on_enter(&mut self) {

    }

    fn update(&mut self, dt: f32, window: &core::Window) -> Option<String> {
        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, dt: f32) {

    }

    fn on_leave(&mut self) {

    }
}