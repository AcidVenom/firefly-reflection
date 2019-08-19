use crate::snuff::core;
use crate::snuff::gfx;

pub struct MenuState {
    camera: core::Camera,
    quad: gfx::Mesh,
    menu_shader: gfx::ShaderProgram,
    title_texture: gfx::Texture2D,
    subtitle_texture: gfx::Texture2D,
    ground_texture: gfx::Texture2D,
    background_texture: gfx::Texture2D,
    title_alpha: f32,
    subtitle_slide: f32,
    slide_down: f32,
    is_first_frame: bool
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
            ground_texture: gfx::Texture2D::from_image(display, "assets/textures/menu/ground.png").with_nearest_filter(),
            background_texture: gfx::Texture2D::from_image(display, "assets/textures/menu/background.png"),
            title_alpha: 0.0,
            subtitle_slide: 0.0,
            slide_down: 0.0,
            is_first_frame: true
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

        if self.is_first_frame {
            self.is_first_frame = false;
            
            return None
        }
        
        // Variables

        let title_speed = 0.66;
        let subtitle_speed = 0.5;
        let slide_down_speed = 0.2;

        if window.is_key_released(glium::glutin::VirtualKeyCode::R) {
            self.title_alpha = 0.0;
            self.subtitle_slide = 0.0;
            self.slide_down = 0.0;
        }

        // Apply animations
        if self.title_alpha < 1.5 {
            self.title_alpha += dt * title_speed;
            self.title_alpha = self.title_alpha.min(1.5);
        }
        else if self.subtitle_slide < 1.0 {
            self.subtitle_slide += dt * subtitle_speed;
            self.subtitle_slide = self.subtitle_slide.min(1.0);
        }
        else if self.slide_down < 1.0 {
            self.slide_down += dt * slide_down_speed;
            self.slide_down = self.slide_down.min(1.0);
        }

        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, dt: f32) {

        // Variables

        let title_offset = 100.0;
        let subtitle_slide = 15.0;
        let parallax_offset = 200.0;

        let title_delayed = self.title_alpha - 0.5;

        let ease_title = core::easing::out_cubic(title_delayed.max(0.0));
        let ease_slide = core::easing::out_cubic(self.subtitle_slide);
        let ease_down = core::easing::in_out_cubic(self.slide_down);
        let fade_out = 1.0 - ease_down;

        // Setup transforms
        let mut transform_background = core::Transform::new();
        transform_background
            .set_size_2d(&self.background_texture.dimensions_f())
            .translate_2d_f(0.0, -360.0 + 720.0 * ease_down);
        
        let mut transform_title = core::Transform::new();
        transform_title
            .set_size_2d(&self.title_texture.dimensions_f())
            .translate_2d_f(0.0, title_offset * 0.75 + title_offset * 0.25 * ease_slide + (1.0 - fade_out) * parallax_offset);

        let mut transform_subtitle = core::Transform::new();
        transform_subtitle
            .set_size_2d(&self.subtitle_texture.dimensions_f())
            .translate_2d_f(0.0, -subtitle_slide + subtitle_slide * ease_slide + (1.0 - fade_out) * parallax_offset);

        let mut transform_ground = core::Transform::new();
        transform_ground
            .set_size_2d(&self.ground_texture.dimensions_f())
            .translate_2d_f(0.0, -720.0 + self.ground_texture.dimensions_f().y * 0.5 + 360.0 * ease_down);

        // Render background
        command_buffer.set_blend_color(1.0, 1.0, 1.0, 1.0);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_background,
            &mut self.menu_shader,
            &vec![&self.background_texture]);

        // Render title
        command_buffer.set_blend_color(1.0, 1.0, 1.0, ease_title * fade_out);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_title,
            &mut self.menu_shader,
            &vec![&self.title_texture]);

        // Render subtitle
        command_buffer.set_blend_color(1.0, 1.0, 1.0, ease_slide * fade_out);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_subtitle,
            &mut self.menu_shader,
            &vec![&self.subtitle_texture]);

        command_buffer.set_blend_color(1.0, 1.0, 1.0, 1.0);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_ground,
            &mut self.menu_shader,
            &vec![&self.ground_texture]);
    }

    fn on_leave(&mut self) {

    }
}