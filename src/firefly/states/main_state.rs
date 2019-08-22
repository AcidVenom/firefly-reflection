use crate::snuff::core;
use crate::snuff::gfx;
use crate::firefly::objects;
use crate::firefly::objects::GameObject;
use rand::prelude::*;

use std::fs::File;
use std::io::BufReader;

#[derive(PartialEq)]
enum FadeMode {
    In,
    Out,
    Done
}

pub struct MainState {
    camera: core::Camera,
    fullscreen_quad: gfx::Mesh,
    quad: gfx::Mesh,
    color_target: gfx::Texture2D,
    background_shader: gfx::ShaderProgram,
    fullscreen_shader: gfx::ShaderProgram,
    shader: gfx::ShaderProgram,
    tree_shader: gfx::ShaderProgram,
    widget_shader: gfx::ShaderProgram,
    fade: f32,
    fade_timer: f32,
    fade_from: f32,
    fade_mode: FadeMode,
    player: objects::Player,
    backgrounds: Vec<objects::BackgroundObject>,
    background_textures: Vec<gfx::Texture2D>,
    trees: Vec<objects::BackgroundObject>,
    tree_textures: Vec<gfx::Texture2D>,
    text_textures: Vec<gfx::Texture2D>,
    text_timer: f32,
    current_text: usize,
    music_sink: rodio::Sink,
    rain_sink: rodio::Sink,
    first_frame: bool,
    audio_swapped: bool,
    raininess: f32
}

impl MainState {
    pub fn new(window: &mut core::Window) -> MainState {
        let display = window.display();
        let audio_device = &rodio::default_output_device().unwrap();

        let mut main_state = MainState {
            camera: core::Camera::new(),
            fullscreen_quad: gfx::Mesh::create_quad(display, false),
            quad: gfx::Mesh::create_quad(display, true),
            color_target: gfx::Texture2D::empty(display, 1280, 720),
            background_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/fullscreen.vs".to_string(), "assets/shaders/background.fs".to_string()).unwrap(),
            fullscreen_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/fullscreen.vs".to_string(), "assets/shaders/fullscreen.fs".to_string()).unwrap(),
            shader: gfx::ShaderProgram::from_source(display, "assets/shaders/simple.vs".to_string(), "assets/shaders/simple.fs".to_string()).unwrap(),
            tree_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/tree.vs".to_string(), "assets/shaders/simple.fs".to_string()).unwrap(),
            widget_shader: gfx::ShaderProgram::from_source(display, "assets/shaders/widget.vs".to_string(), "assets/shaders/widget.fs".to_string()).unwrap(),
            fade: 1.0,
            fade_timer: 0.0,
            fade_from: 1.0,
            fade_mode: FadeMode::In,
            player: objects::Player::new(display),
            backgrounds: Vec::new(),
            background_textures: Vec::new(),
            trees: Vec::new(),
            tree_textures: Vec::new(),
            text_textures: Vec::new(),
            text_timer: 0.0,
            current_text: 0,
            music_sink: rodio::Sink::new(audio_device),
            rain_sink: rodio::Sink::new(audio_device),
            first_frame: true,
            audio_swapped: false,
            raininess: 1.0
        };

        let num_backgrounds = 7;
        let background_offset = main_state.player.border().y + 160.0;

        for i in 0..num_backgrounds {
            let mut background = objects::BackgroundObject {
                background_index: i,
                transform: core::Transform::new()
            };

            background.transform
                .set_size_2d_f(1280.0, 720.0)
                .set_translation_2d_f((i as f32) * 1280.0, background_offset);

            main_state.backgrounds.push(background);
            main_state.background_textures.push(
                gfx::Texture2D::from_image(display, &format!("assets/textures/backgrounds/{}.png", i)[..]).with_nearest_filter());
        }

        let num_tree_textures = 4;
        for i in 0..num_tree_textures {
            main_state.tree_textures.push(
                gfx::Texture2D::from_image(display, &format!("assets/textures/trees/{}.png", i)[..]).with_nearest_filter());
        }

        let num_text_textures = 16;
        for i in 0..num_text_textures {
            main_state.text_textures.push(
                gfx::Texture2D::from_image(display, &format!("assets/textures/story/{}.png", i)[..]).with_nearest_filter());
        }

        let num_trees = 80;
        let mut tree_offset = 300.0;
        let min_offset = 10.0;
        let max_offset = 30.0;
        let f_num_trees = num_tree_textures as f32;

        let mut rng = rand::thread_rng();
        for _ in 0..num_trees {
            let mut background_index: f32 = rng.gen();
            background_index = background_index * f_num_trees;

            let mut tree = objects::BackgroundObject {
                background_index: background_index as usize,
                transform: core::Transform::new()
            };

            let mut size: f32 = rng.gen();
            size = 1.0 - size * 0.4;

            tree.transform
                .set_translation_2d_f(tree_offset, main_state.player.border().y)
                .set_anchor_2d_f(0.0, 0.5)
                .set_size_2d(&(main_state.tree_textures[tree.background_index].dimensions_f() * size))
                .set_scale_2d_f(if rand::random() { -1.0 } else { 1.0 }, 1.0);

            let random_offset: f32 = rng.gen();
            tree_offset += min_offset + (max_offset - min_offset) * random_offset;

            main_state.trees.push(tree);
        }

        main_state.camera
            .set_orthographic(true)
            .set_orthographic_size_both_f(1280.0, 720.0)
            .transform().set_translation_f(0.0, 0.0, -1.0);

        main_state
    }
}

impl MainState {
    fn set_defaults(&mut self) {
        self.fade = 1.0;
        self.fade_in();

        let audio_device = &rodio::default_output_device().unwrap();
        
        let audio_file = File::open("assets/audio/music_mid.mp3").unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(audio_file)).unwrap();

        self.music_sink = rodio::Sink::new(audio_device);
        self.music_sink.append(audio_source);

        let audio_file = File::open("assets/audio/rain.mp3").unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(audio_file)).unwrap();

        self.rain_sink = rodio::Sink::new(audio_device);
        self.rain_sink.append(audio_source);
    }

    fn play_end_music(&mut self) {
        let audio_file = File::open("assets/audio/music_end.mp3").unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(audio_file)).unwrap();

        self.music_sink = rodio::Sink::new(&rodio::default_output_device().unwrap());
        self.music_sink.append(audio_source);

        self.audio_swapped = true;
    }

    fn fade_in(&mut self) {
        self.fade_mode = FadeMode::In;
        self.fade_from = self.fade;
        self.fade_timer = 0.0;
    }

    fn fade_out(&mut self) {
        self.fade_mode = FadeMode::Out;
        self.fade_from = self.fade;
        self.fade_timer = 0.0;
    }

    fn update_fade(&mut self, dt: f32) {

        if self.fade_mode != FadeMode::Done {
            let fade_duration = 2.0;
            self.fade_timer += dt * (1.0 / fade_duration);
            self.fade_timer = self.fade_timer.min(1.0);
        }

        let ease = core::easing::out_cubic(self.fade_timer);
        
        match &self.fade_mode {
            FadeMode::In => {
                self.fade = self.fade_from * (1.0 - ease);
            },
            FadeMode::Out => {
                self.fade = self.fade_from * (1.0 - ease) + ease;
            },
            FadeMode::Done => {}
        }
    }
}

impl core::GameState for MainState {
    fn on_enter(&mut self) {
        self.set_defaults();
    }

    fn update(&mut self, dt: f32, window: &core::Window) -> Option<String> {

        if self.first_frame == true {

            self.music_sink.play();
            self.rain_sink.play();

            self.first_frame = false;
            return None;
        }

        // Variables
        let camera_damping = 100.0;
        let camera_offset = nalgebra_glm::vec2(0.0, 100.0);

        // Debug
        
        let dt = if window.is_key_down(glium::glutin::VirtualKeyCode::RBracket) { dt * 10.0 } else { dt };

        if window.is_key_released(glium::glutin::VirtualKeyCode::R) {
            return Some(String::from("MenuState"));
        }
        else if window.is_key_released(glium::glutin::VirtualKeyCode::F) {
            self.fade_out();
        }
        else if window.is_key_released(glium::glutin::VirtualKeyCode::G) {
            self.fade_in();
        }

        // Fading

        self.update_fade(dt);
        self.player.update(dt, window);

        let player_x = self.player.transform().translation().x;
        let at_end = self.player.is_at_end();

        if player_x - 640.0 > self.current_text as f32 * 1280.0 || at_end && self.text_timer >= 1.0 {
            self.text_timer = 0.0;
            self.current_text += 1;
        }

        // "Cross-fade" the audio
        let border = self.player.border();
        let volume_ratio = if at_end && self.current_text > 6 { 1.0 } else { 1.0 - ((player_x - border.x) / (border.z - border.x)).powf(2.0) };

        if at_end && !self.audio_swapped && self.current_text > 6 {
            self.play_end_music();
        }

        self.music_sink.set_volume(volume_ratio);

        // Rain

        if self.current_text >= 13 && self.raininess > 0.0 {
            self.raininess -= dt * (1.0 / 14.0);
            self.raininess = self.raininess.max(0.0);

            self.rain_sink.set_volume(self.raininess);
        }

        // Follow a point around with the camera

        let mut t = self.camera.transform().translation_2d();
        t = nalgebra_glm::lerp(&t, &(self.player.transform().translation_2d() + camera_offset), 1.0 - f32::powf(1.0 / camera_damping, dt));
        t.x = t.x.max(0.0);
        self.camera.transform().set_translation_2d(&t);

        // Text

        let text_duration = 6.5;
        if self.text_timer < 1.0 {
            self.text_timer += dt * (1.0 / text_duration);
            self.text_timer = self.text_timer.min(1.0);
        }

        None
    }

    fn draw(&mut self, command_buffer: &mut gfx::CommandBuffer, _dt: f32) {

        let mut fullscreen_transform = core::Transform::new();
        let mut text_transform = core::Transform::new();

        let text_ease = core::easing::out_cubic(self.text_timer);
        text_transform
            .set_size_2d(&self.text_textures[0].dimensions_f())
            .set_translation_2d_f(0.0, 170.0 + 30.0 * text_ease);

        // Draw background
        command_buffer.set_blend_color(1.0, 1.0, 1.0, self.raininess);
        let mut target = command_buffer.render_target(vec![&self.color_target]);
        command_buffer.clear(&mut target, 0.0, 0.0, 0.0, 0.0);

        command_buffer.draw_into_target(
            &mut target,
            &mut self.camera,
            &self.fullscreen_quad,
            &mut fullscreen_transform,
            &mut self.background_shader,
            &Vec::new());

        drop(target);

        command_buffer.set_blend_color(1.0, 1.0, 1.0, 1.0);
        let mut target = command_buffer.render_target(vec![&self.color_target]);

        for it in self.trees.iter_mut() {
            command_buffer.draw_into_target(
                &mut target,
                &mut self.camera,
                &self.quad,
                &mut it.transform,
                &mut self.tree_shader,
                &vec![&self.tree_textures[it.background_index]]);
        }

        for it in self.backgrounds.iter_mut() {
            command_buffer.draw_into_target(
                &mut target,
                &mut self.camera,
                &self.quad,
                &mut it.transform,
                &mut self.shader,
                &vec![&self.background_textures[it.background_index]]);
        }

        // Render player

        let render_data = self.player.render_data();
        command_buffer.draw_into_target(
            &mut target,
            &mut self.camera,
            &self.quad,
            render_data.transform,
            &mut self.shader,
            &render_data.textures);

        // Final pass
        command_buffer.set_blend_color(1.0, 1.0, 1.0, 1.0 - self.fade);
        command_buffer.fullscreen_pass(
            &mut self.camera,
            &mut self.fullscreen_shader, 
            &vec![&self.color_target]);

        // Render texts
        if self.current_text < self.text_textures.len() {
            let c = (self.text_timer * 3.14159).sin();
            command_buffer.set_blend_color(1.0, 1.0, 1.0, c);
            command_buffer.draw(&mut self.camera, &self.quad, &mut text_transform, &mut self.widget_shader, &vec![&self.text_textures[self.current_text]]);
        }
    }

    fn on_leave(&mut self) {

    }
}