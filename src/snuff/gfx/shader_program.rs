use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct ShaderProgram {
    program: glium::Program,
    watcher: RecommendedWatcher,
    watch_receiver: std::sync::mpsc::Receiver<notify::DebouncedEvent>,
    vs_file_path: String,
    fs_file_path: String
}

impl ShaderProgram {
    //---------------------------------------------------------------------------------------------------
    pub fn from_string<'a>(
        display: &glium::Display,
        vs_raw: &'a str,
        fs_raw: &'a str,
    ) -> Result<ShaderProgram, glium::ProgramCreationError> {
        let (sender, receiver) = channel();

        match glium::Program::new(display, glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: vs_raw,
            fragment_shader: fs_raw,
            geometry_shader: None,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: false,
        }) {
            Ok(program) => Ok(ShaderProgram {
                program,
                watcher: Watcher::new(sender, Duration::from_millis(150)).unwrap(),
                watch_receiver: receiver,
                vs_file_path: String::from("null"),
                fs_file_path: String::from("null")
            }),
            Err(e) => {
                println!("[ShaderProgram] Could not create shader program: {}", e);
                Err(e)
            }
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn from_source<'a>(
        display: &glium::Display,
        vs_file: String,
        fs_file: String,
    ) -> Result<ShaderProgram, glium::ProgramCreationError> {
        let vs_contents = std::fs::read_to_string(vs_file.to_string()).expect(
            &format!(
                "[ShaderProgram] Could not open vertex shader file '{}'",
                vs_file
            )[..],
        );

        let fs_contents = std::fs::read_to_string(fs_file.to_string()).expect(
            &format!(
                "[ShaderProgram] Could not open fragment shader file '{}'",
                fs_file
            )[..],
        );

        let new_program = ShaderProgram::from_string(display, &vs_contents, &fs_contents);

        match new_program
        {
            Ok(mut shader_program) => {
                let _ = shader_program.watcher.watch(vs_file.to_string(), RecursiveMode::NonRecursive);
                let _ = shader_program.watcher.watch(fs_file.to_string(), RecursiveMode::NonRecursive);

                shader_program.vs_file_path = vs_file;
                shader_program.fs_file_path = fs_file;

                Ok(shader_program)
            },
            Err(err) => Err(err)
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn reload(&mut self, display: &glium::Display) {
        let new_shader = ShaderProgram::from_source(display, self.vs_file_path.to_string(), self.fs_file_path.to_string());

        match new_shader {
            Ok(shader_program) => { self.program = shader_program.program; println!("[ShaderProgram] Reloaded ({}, {})", self.vs_file_path, self.fs_file_path); },
            Err(err) => { println!("[ShaderProgram] Could not reload shader ('{}', '{}'): {}", self.vs_file_path, self.fs_file_path, err); }
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn check_reload(&mut self, display: &glium::Display) {
        match self.watch_receiver.try_recv() {
            Ok(evt) => { 
                match evt {
                    notify::DebouncedEvent::Write(_) => {
                        self.reload(display)
                    },
                    _ => {}
                }
             }
            _ => {}
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn program(&mut self, display: &glium::Display) -> &glium::Program {
        self.check_reload(display);
        &self.program
    }
}
