use crate::snuff::core;
use crate::snuff::gfx;
use crate::firefly::objects;

pub struct Player {
    texture: gfx::Texture2D,
    transform: core::Transform,
    velocity: nalgebra_glm::Vec2,
    grounded: bool
}

impl objects::GameObject for Player {
    fn render_data(&mut self) -> objects::RenderData {
        objects::RenderData {
            textures: vec![&self.texture],
            transform: &mut self.transform
        }
    }
}

impl Player {
    pub fn new(display: &glium::Display) -> Player {
        let mut player = Player {
            texture: gfx::Texture2D::from_image(display, "assets/textures/characters/player.png"),
            transform: core::Transform::new(),
            velocity: nalgebra_glm::vec2(0.0, 0.0),
            grounded: false
        };

        player.transform.set_size_2d(&player.texture.dimensions_f());

        player
    }

    pub fn texture(&mut self) -> &gfx::Texture2D {
        &self.texture
    }

    pub fn transform(&mut self) -> &mut core::Transform {
        &mut self.transform
    }

    pub fn update(&mut self, dt: f32, window: &core::Window) {

        let fall_speed = 1000.0;
        let jump_force = 500.0;
        let border = -300.0;

        let mut t = self.transform.translation_2d() + self.velocity * dt;

        if window.is_key_down(glium::glutin::VirtualKeyCode::Space) && self.grounded {
            self.velocity.y = jump_force;
            self.grounded = false;
        }

        if t.y > border {
            self.velocity.y -= dt * fall_speed;
        }

        if t.y < border {
            t.y = border;
            self.velocity.y = 0.0;
            self.grounded = true;
        }

        self.transform.set_translation_2d(&t);
    }
}