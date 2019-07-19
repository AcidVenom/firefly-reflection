use crate::snuff;

use glium;
use glium::Surface;

pub struct CommandBuffer {
    frame: glium::Frame,
}

impl CommandBuffer {
    //---------------------------------------------------------------------------------------------------
    fn matrix_to_uniform(m: &nalgebra_glm::Mat4) -> [[f32; 4]; 4] {
        *m.as_ref()
    }

    //---------------------------------------------------------------------------------------------------
    pub fn new(display: &glium::Display) -> CommandBuffer {
        let mut target = display.draw();
        target.clear_color(0.1, 0.33, 1.0, 1.0);

        CommandBuffer { frame: target }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn draw(
        &mut self,
        camera: &mut snuff::core::Camera,
        mesh: &snuff::gfx::Mesh,
        transform: &mut snuff::core::Transform,
        shader: &snuff::gfx::ShaderProgram,
        textures: &mut Vec<&snuff::gfx::Texture2D>
    ) {
        let filtering = textures[0].filtering();
        let uniforms = uniform! {
            model: CommandBuffer::matrix_to_uniform(&transform.local_to_world()),
            view: CommandBuffer::matrix_to_uniform(&camera.view()),
            projection: CommandBuffer::matrix_to_uniform(&camera.projection()),
            tex0: glium::uniforms::Sampler(textures[0].texture(), glium::uniforms::SamplerBehavior {
                wrap_function: (glium::uniforms::SamplerWrapFunction::Repeat, glium::uniforms::SamplerWrapFunction::Repeat, glium::uniforms::SamplerWrapFunction::Repeat),
                minify_filter: filtering.0,
                magnify_filter: filtering.1,
                depth_texture_comparison: None,
                max_anisotropy: 1,
            })
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
