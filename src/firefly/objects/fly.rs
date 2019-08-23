use crate::snuff::gfx;
use crate::snuff::core;
use crate::firefly::objects;

pub struct Firefly {
    transform: core::Transform,
    textures: Vec<gfx::Texture2D>,
    current_texture: usize,
    flip_timer: f32,
    offset_timer: f32,
    started_following: bool
}

impl objects::GameObject for Firefly {
    fn render_data(&mut self) -> objects::RenderData {
        objects::RenderData {
            textures: vec![&self.textures[self.current_texture]],
            transform: &mut self.transform
        }
    }
}

impl Firefly {

    pub fn new(display: &glium::Display) -> Firefly {
        let mut firefly = Firefly {
            transform: core::Transform::new(),
            textures: Vec::new(),
            current_texture: 0,
            flip_timer: 0.0,
            offset_timer: 0.0,
            started_following: false
        };

        firefly.textures.push(gfx::Texture2D::from_image(display, "assets/textures/characters/firefly_up.png"));
        firefly.textures.push(gfx::Texture2D::from_image(display, "assets/textures/characters/firefly_down.png"));

        firefly.transform.set_size_2d(&(firefly.textures[0].dimensions_f() * 0.25));

        firefly
    }

    pub fn update(&mut self, player_transform: &core::Transform, end_offset: f32, dt: f32) {
        let player_x = player_transform.translation().x;
        if player_x < 2.5 * 1280.0 && !self.started_following {
            self.transform.set_translation_2d_f(player_x, 900.0);
            return;
        }

        self.started_following = true;

        let damping = 3.0;

        self.offset_timer += dt * 0.25;
        if self.offset_timer > 1.0 {
            self.offset_timer = 0.0;
        }

        let twirl = 100.0;
        let twirl_ratio = self.offset_timer * 6.28;
        let horizontal_offset = twirl_ratio.sin() * twirl;
        let vertical_offset = twirl_ratio.cos() * twirl * 0.25;
        let diverge = if player_x > 3.75 * 1280.0 && player_x < 4.5 * 1280.0 { 550.0 } else { 0.0 };
        
        let offset = nalgebra_glm::vec2(horizontal_offset, 75.0 + vertical_offset + diverge + end_offset);

        let mut t = self.transform.translation_2d();
        t = nalgebra_glm::lerp(&t, &(player_transform.translation_2d() + offset), 1.0 - f32::powf(1.0 / damping, dt));

        self.transform.set_translation_2d(&t);

        let flutter_speed = 20.0;
        self.flip_timer += dt * flutter_speed;

        while self.flip_timer >= 1.0 {
            self.flip_timer -= 1.0;
            self.current_texture = if self.current_texture == 0 { 1 } else { 0 };
        }
    }
}