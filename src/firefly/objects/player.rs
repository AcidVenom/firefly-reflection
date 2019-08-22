use crate::snuff::core;
use crate::snuff::gfx;
use crate::firefly::objects;

enum PlayerState {
    Initial,
    User
}

pub struct Player {
    state: PlayerState,
    texture: gfx::Texture2D,
    transform: core::Transform,
    velocity: nalgebra_glm::Vec2,
    grounded: bool,
    hop_force: f32,
    acceleration: f32,
    squish_timer: f32
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
            state: PlayerState::Initial,
            texture: gfx::Texture2D::from_image(display, "assets/textures/characters/player.png"),
            transform: core::Transform::new(),
            velocity: nalgebra_glm::vec2(0.0, 0.0),
            grounded: false,
            hop_force: 300.0,
            acceleration: 1500.0,
            squish_timer: 1.0
        };

        player.transform
            .set_size_2d(&player.texture.dimensions_f())
            .set_translation_2d_f(-700.0, -300.0)
            .set_anchor_2d_f(0.0, 0.25);

        player
    }

    pub fn texture(&mut self) -> &gfx::Texture2D {
        &self.texture
    }

    pub fn transform(&mut self) -> &mut core::Transform {
        &mut self.transform
    }

    fn update_animations(&mut self, dt: f32) {

        // Variables
        let pi = 3.14159;
        let squish_scale = 0.175;
        let squish_duration = 0.25;

        // Flipping
        let mut speed_scale = 1.0;
        if self.velocity.x != 0.0 {
            speed_scale = self.velocity.x.abs() / self.velocity.x;
        }
        
        // Hopping
        let hop_ratio = self.velocity.y.max(0.0).min(300.0) / 300.0;
        let hop_scale = 1.0 + (hop_ratio * pi * 2.0).sin() * 0.125;
        let hop_rot = (hop_ratio * pi).sin() * -0.25;

        // Squishing
        if self.squish_timer < 1.0 {
            self.squish_timer += dt * (1.0 / squish_duration);
            self.squish_timer = self.squish_timer.min(1.0);
        }

        let hop_squish = (self.squish_timer * pi).sin() * squish_scale;

        // Apply
        self.transform
            .set_scale_2d_f(speed_scale * (1.0 + hop_squish), hop_scale - hop_squish * 0.5)
            .set_orientation(hop_rot);
    }

    pub fn hop_right(&mut self)
    {
        if !self.grounded {
            return;
        }

        if self.velocity.x < 0.0 {
            self.velocity.x *= 0.75;
        }
        self.velocity.x += self.acceleration;
        self.velocity.y = self.hop_force;
        self.grounded = false;
    }

    pub fn hop_left(&mut self)
    {
        if !self.grounded {
            return;
        }

        if self.velocity.x > 0.0 {
            self.velocity.x *= 0.75;
        }
        self.velocity.x -= self.acceleration;
        self.velocity.y = self.hop_force;
        self.grounded = false;
    }

    pub fn damp_velocity(&mut self, dt: f32) {
        let damping = 50.0;
        self.velocity.x = nalgebra_glm::lerp_scalar(self.velocity.x, 0.0, 1.0 - f32::powf(1.0 / damping, dt));
    }

    pub fn update_velocity(&mut self, dt: f32) {
        // Variables

        let fall_speed = 1000.0;
        let border = -300.0;

        let mut t = self.transform.translation_2d() + self.velocity * dt;

        // Make sure we don't fall through the ground..
        if t.y <= border && !self.grounded {
            t.y = border;
            self.velocity.y = 0.0;
            self.grounded = true;
            self.squish_timer = 0.0;
        }

        if self.grounded {
            self.damp_velocity(dt);
        }
        else {
            self.velocity.y -= dt * fall_speed;
        }
        
        // Set the new translation
        self.transform.set_translation_2d(&t);
    }

    pub fn clamp_velocity(&mut self) {
        let max_speed = 300.0;
        let max_fall_speed = 2000.0;

        self.velocity = nalgebra_glm::clamp_vec(
            &self.velocity, 
            &nalgebra_glm::vec2(-max_speed, -max_fall_speed), 
            &nalgebra_glm::vec2(max_speed, self.hop_force));
    }

    pub fn update_input(&mut self, window: &core::Window) {
        if window.is_key_down(glium::glutin::VirtualKeyCode::Right) {
            self.hop_right();
        }
        else if window.is_key_down(glium::glutin::VirtualKeyCode::Left) {
            self.hop_left();
        }
    }

    pub fn initial_state(&mut self) {
        // Hop right once, then give control to the user
        self.hop_right();
        self.state = PlayerState::User;
    }

    pub fn update(&mut self, dt: f32, window: &core::Window) {

        // Constant accelerations
        self.update_velocity(dt);

        match &self.state {
            PlayerState::Initial => {
                self.initial_state();
            },
            PlayerState::User => {
                // Input
                self.update_input(window);
            }
        }

        // Clamp before end of frame
        self.clamp_velocity();

        // Update all animations accordingly
        self.update_animations(dt);
    }
}