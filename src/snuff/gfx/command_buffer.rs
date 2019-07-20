use crate::snuff;

use glium;
use glium::Surface;

pub struct CommandBuffer<'a> {
    frame: glium::Frame,
    default_texture: &'a snuff::gfx::Texture2D
}

impl<'a> CommandBuffer<'a> {

    //---------------------------------------------------------------------------------------------------
    pub fn new(display: &glium::Display, default_texture : &'a snuff::gfx::Texture2D) -> CommandBuffer<'a> {
        let mut target = display.draw();
        target.clear_color(0.1, 0.33, 1.0, 1.0);

        CommandBuffer { 
            frame: target,
            default_texture
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn create_sampler_uniform(&self, index : usize, textures : &'a Vec<&snuff::gfx::Texture2D>) -> glium::uniforms::Sampler<'a, glium::texture::Texture2d> {
        let texture_handle = if index >= textures.len() { self.default_texture } else { textures[index] };

        let texture = texture_handle.texture();
        let filtering = texture_handle.filtering();

        glium::uniforms::Sampler::new(texture)
            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
            .minify_filter(filtering.0)
            .magnify_filter(filtering.1)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn draw(
        &mut self,
        camera: &mut snuff::core::Camera,
        mesh: &snuff::gfx::Mesh,
        transform: &mut snuff::core::Transform,
        shader: &snuff::gfx::ShaderProgram,
        textures: &Vec<&snuff::gfx::Texture2D>
    ) {
        let uniforms = uniform! {
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
    pub fn end(self) {
        match self.frame.finish() {
            Ok(_) => {}
            Err(e) => println!("[CommandBuffer] Could not swap buffers in 'end' : {}", e),
        }
    }
}
