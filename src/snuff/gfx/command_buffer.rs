use crate::snuff;

use glium;
use glium::Surface;

pub struct CommandBuffer<'a> {
    frame: glium::Frame,
    display: &'a glium::Display,
    default_texture: &'a snuff::gfx::Texture2D,
    fullscreen_quad: &'a snuff::gfx::Mesh,
    time: f32,
}

pub struct RenderTarget<'a> {
    framebuffer: glium::framebuffer::MultiOutputFrameBuffer<'a>,
    textures: Vec<&'a snuff::gfx::Texture2D>,
}

impl<'a> CommandBuffer<'a> {
    //---------------------------------------------------------------------------------------------------
    pub fn new(
        display: &'a glium::Display,
        default_texture: &'a snuff::gfx::Texture2D,
        fullscreen_quad: &'a snuff::gfx::Mesh,
        time: f32,
    ) -> CommandBuffer<'a> {
        let mut target = display.draw();
        target.clear_color(0.1, 0.33, 1.0, 1.0);

        CommandBuffer {
            frame: target,
            display: display,
            default_texture,
            fullscreen_quad,
            time: time,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn create_sampler_uniform(
        &self,
        index: usize,
        textures: &'a Vec<&snuff::gfx::Texture2D>,
    ) -> glium::uniforms::Sampler<'a, glium::texture::Texture2d> {
        let texture_handle = if index >= textures.len() {
            self.default_texture
        } else {
            textures[index]
        };

        let texture = texture_handle.texture();
        let filtering = texture_handle.filtering();

        glium::uniforms::Sampler::new(texture)
            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
            .minify_filter(filtering.0)
            .magnify_filter(filtering.1)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn render_target<'b>(&self, textures: Vec<&'a snuff::gfx::Texture2D>) -> RenderTarget {
        assert!(textures.len() <= 4, "[CommandBuffer] Cannot create a render target with more than 4 output values");

        let mut outputs : Vec<(&'a str, &glium::texture::Texture2d)> = Vec::new();

        let names = vec!["output0", "output1", "output2", "output3"];

        let mut output_count = 0;
        for it in textures.iter() {
            outputs.push((names[output_count], it.texture()));
            output_count += 1;
        }
        
        RenderTarget {
            framebuffer: glium::framebuffer::MultiOutputFrameBuffer::new(
                self.display,
                outputs.into_iter(),
            )
            .unwrap(),
            textures,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn clear(&self, target: &mut RenderTarget, r: f32, g: f32, b: f32, a: f32) {
        target.framebuffer.clear_color(r, g, b, a);
    }

    //---------------------------------------------------------------------------------------------------
    pub fn draw(
        &mut self,
        camera: &mut snuff::core::Camera,
        mesh: &snuff::gfx::Mesh,
        transform: &mut snuff::core::Transform,
        shader: &snuff::gfx::ShaderProgram,
        textures: &Vec<&snuff::gfx::Texture2D>,
    ) {
        let uniforms = uniform! {
            time: self.time,
            model: *transform.local_to_world().as_ref(),
            view: *camera.view().as_ref(),
            projection: *camera.projection().as_ref(),
            sampler0: self.create_sampler_uniform(0, textures),
            sampler1: self.create_sampler_uniform(1, textures),
            sampler2: self.create_sampler_uniform(2, textures),
            sampler3: self.create_sampler_uniform(3, textures),
            sampler4: self.create_sampler_uniform(4, textures),
            sampler5: self.create_sampler_uniform(5, textures),
            sampler6: self.create_sampler_uniform(6, textures),
            sampler7: self.create_sampler_uniform(7, textures)
        };

        self.frame
            .draw(
                mesh.vertex_buffer(),
                mesh.index_buffer(),
                shader.program(),
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    //---------------------------------------------------------------------------------------------------
    pub fn draw_into_target(
        &self,
        target: &mut RenderTarget,
        camera: &mut snuff::core::Camera,
        mesh: &snuff::gfx::Mesh,
        transform: &mut snuff::core::Transform,
        shader: &snuff::gfx::ShaderProgram,
        textures: &Vec<&snuff::gfx::Texture2D>,
    ) {
        let uniforms = uniform! {
            time: self.time,
            model: *transform.local_to_world().as_ref(),
            view: *camera.view().as_ref(),
            projection: *camera.projection().as_ref(),
            sampler0: self.create_sampler_uniform(0, textures),
            sampler1: self.create_sampler_uniform(1, textures),
            sampler2: self.create_sampler_uniform(2, textures),
            sampler3: self.create_sampler_uniform(3, textures),
            sampler4: self.create_sampler_uniform(4, textures),
            sampler5: self.create_sampler_uniform(5, textures),
            sampler6: self.create_sampler_uniform(6, textures),
            sampler7: self.create_sampler_uniform(7, textures)
        };

        target
            .framebuffer
            .draw(
                mesh.vertex_buffer(),
                mesh.index_buffer(),
                shader.program(),
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    //---------------------------------------------------------------------------------------------------
    pub fn fullscreen_pass(
        &mut self,
        camera: &mut snuff::core::Camera,
        shader: &snuff::gfx::ShaderProgram,
        textures: &Vec<&snuff::gfx::Texture2D>,
    ) {
        let uniforms = uniform! {
            time: self.time,
            view: *camera.view().as_ref(),
            projection: *camera.projection().as_ref(),
            sampler0: self.create_sampler_uniform(0, textures),
            sampler1: self.create_sampler_uniform(1, textures),
            sampler2: self.create_sampler_uniform(2, textures),
            sampler3: self.create_sampler_uniform(3, textures),
            sampler4: self.create_sampler_uniform(4, textures),
            sampler5: self.create_sampler_uniform(5, textures),
            sampler6: self.create_sampler_uniform(6, textures),
            sampler7: self.create_sampler_uniform(7, textures)
        };

        self.frame
            .draw(
                self.fullscreen_quad.vertex_buffer(),
                self.fullscreen_quad.index_buffer(),
                shader.program(),
                &uniforms,
                &Default::default(),
            )
            .unwrap()
    }

    //---------------------------------------------------------------------------------------------------
    pub fn end(self) {
        match self.frame.finish() {
            Ok(_) => {}
            Err(e) => println!("[CommandBuffer] Could not swap buffers in 'end' : {}", e),
        }
    }
}
