use crate::snuff;

use glium;
use glium::Surface;

pub struct CommandBuffer {
    frame : glium::Frame
}

impl CommandBuffer {
    //---------------------------------------------------------------------------------------------------
    fn matrix_to_uniform(m : &nalgebra_glm::Mat4) -> [[f32; 4]; 4] {
        let c1 = m.column(0);
        let c2 = m.column(1);
        let c3 = m.column(2);
        let c4 = m.column(3);

        [
            [c1[0], c1[1], c1[2], c1[3]],
            [c2[0], c2[1], c2[2], c2[3]],
            [c3[0], c3[1], c3[2], c3[3]],
            [c4[0], c4[1], c4[2], c4[3]]
        ]
    }
    
    //---------------------------------------------------------------------------------------------------
    pub fn new(display : &glium::Display) -> CommandBuffer {
        let mut target = display.draw();
        target.clear_color(0.1, 0.33, 1.0, 1.0);

        CommandBuffer {
            frame : target
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn draw(&mut self, 
        camera : &mut snuff::core::Camera, 
        mesh : &snuff::gfx::Mesh, 
        transform : &mut snuff::core::Transform, 
        shader : &snuff::gfx::ShaderProgram) {

        let uniforms = uniform! {
            model: CommandBuffer::matrix_to_uniform(&transform.local_to_world()),
            view: CommandBuffer::matrix_to_uniform(&camera.view()),
            projection: CommandBuffer::matrix_to_uniform(&camera.projection())
        };

        self.frame.draw(
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
            Err(e) => println!("[CommandBuffer] Could not swap buffers in 'end' : {}", e)
        }
    }
}