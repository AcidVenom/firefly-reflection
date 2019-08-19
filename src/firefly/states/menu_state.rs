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
    player_texture: gfx::Texture2D,
    title_alpha: f32,
    subtitle_slide: f32,
    slide_down: f32,
    player_hop: f32,
    fade_to_black: f32,
    is_first_frame: bool,
    player_transform: core::Transform
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
            player_texture: gfx::Texture2D::from_image(display, "assets/textures/characters/player.png"),
            title_alpha: 0.0,
            subtitle_slide: 0.0,
            slide_down: 0.0,
            player_hop: 0.0,
            fade_to_black: 0.0,
            is_first_frame: true,
            player_transform: core::Transform::new()
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

        // Skip first frame because of high delta-times

        if self.is_first_frame {
            self.is_first_frame = false;
            
            return None
        }

        // Debug

        let mut speed = dt;
        if window.is_key_down(glium::glutin::VirtualKeyCode::RBracket) {
            speed = speed * 10.0;
        }
        else if window.is_key_down(glium::glutin::VirtualKeyCode::LBracket) {
            speed = speed * 0.1;
        }

        if window.is_key_released(glium::glutin::VirtualKeyCode::R) {
            self.title_alpha = 0.0;
            self.subtitle_slide = 0.0;
            self.slide_down = 0.0;
            self.player_hop = 0.0;
            self.fade_to_black = 0.0;
        }
        
        // Variables

        let title_speed = 0.2;
        let subtitle_speed = 0.33;
        let slide_down_speed = 0.125;
        let player_y_offset = -200.0;
        let player_pixels_per_second = 200.0;
        let to_hop = 1440.0;
        let hop_height = 40.0;

        // Apply animations
        if self.title_alpha < 1.0 {
            self.title_alpha += speed * title_speed;
            self.title_alpha = self.title_alpha.min(1.0);
        }
        else if self.subtitle_slide < 1.0 {
            self.subtitle_slide += speed * subtitle_speed;
            self.subtitle_slide = self.subtitle_slide.min(1.0);
        }
        else {
            self.slide_down += speed * slide_down_speed;
            self.slide_down = self.slide_down.min(1.0);

            if self.slide_down > 0.5 {
                self.player_hop += speed * player_pixels_per_second;
            }

            if self.player_hop > to_hop {
                self.fade_to_black += speed;
                self.fade_to_black = self.fade_to_black.min(1.0);
            }
        }
        

        let pi = 3.14159;
        let hop_factor = f32::abs(f32::sin(self.player_hop / player_pixels_per_second * pi * 1.75));
        let squish_factor = hop_factor.powf(0.75);
        let ease_down = core::easing::in_out_cubic(self.slide_down);

        self.player_transform
            .set_anchor_2d_f(0.0, 0.33)
            .set_size_2d(&self.player_texture.dimensions_f())
            .set_translation_2d_f(-to_hop * 0.5 + self.player_hop, hop_factor * hop_height)
            .translate_2d_f(0.0, player_y_offset - 360.0 + ease_down * 360.0)
            .set_scale_2d_f(1.0 + (1.0 - squish_factor) * 0.5, 0.25 + squish_factor * 0.9)
            .set_orientation(0.1 - hop_factor * 0.3);

        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, dt: f32) {

        // Variables

        let title_offset = 100.0;
        let subtitle_slide = 15.0;
        let parallax_offset = 150.0;

        let ease_title = core::easing::in_out_quad(self.title_alpha);
        let ease_slide = core::easing::in_out_cubic(self.subtitle_slide);
        let ease_down = core::easing::in_out_cubic(self.slide_down);
        let fade_out = (1.0 - self.slide_down * 1.75).max(0.0);

        // Setup transforms
        let mut transform_background = core::Transform::new();
        transform_background
            .set_size_2d(&self.background_texture.dimensions_f())
            .translate_2d_f(0.0, -360.0 + 720.0 * ease_down);
        
        let mut transform_title = core::Transform::new();
        transform_title
            .set_size_2d(&self.title_texture.dimensions_f())
            .translate_2d_f(0.0, 
                title_offset * 0.35 +
                title_offset * 0.4 * ease_title + 
                title_offset * 0.25 * ease_slide + 
                ease_down * parallax_offset);

        let mut transform_subtitle = core::Transform::new();
        transform_subtitle
            .set_size_2d(&self.subtitle_texture.dimensions_f())
            .translate_2d_f(0.0, 
                -subtitle_slide + subtitle_slide * ease_slide + 
                ease_down * parallax_offset);

        let mut transform_ground = core::Transform::new();
        transform_ground
            .set_size_2d(&self.ground_texture.dimensions_f())
            .translate_2d_f(0.0, -720.0 + self.ground_texture.dimensions_f().y * 0.5 + 360.0 * ease_down);

        let c = core::easing::out_cubic(1.0 - self.fade_to_black);

        // Render background
        command_buffer.set_blend_color(c, c, c, 1.0);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_background,
            &mut self.menu_shader,
            &vec![&self.background_texture]);

        // Render title
        command_buffer.set_blend_color(c, c, c, ease_title * fade_out);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_title,
            &mut self.menu_shader,
            &vec![&self.title_texture]);

        // Render subtitle
        command_buffer.set_blend_color(c, c, c, ease_slide * fade_out);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_subtitle,
            &mut self.menu_shader,
            &vec![&self.subtitle_texture]);

        // Render ground
        command_buffer.set_blend_color(c, c, c, 1.0);
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut transform_ground,
            &mut self.menu_shader,
            &vec![&self.ground_texture]);

        // Render player
        command_buffer.draw(
            &mut self.camera,
            &self.quad,
            &mut self.player_transform,
            &mut self.menu_shader,
            &vec![&self.player_texture]);
    }

    fn on_leave(&mut self) {

    }
}