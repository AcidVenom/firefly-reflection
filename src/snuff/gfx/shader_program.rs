pub struct ShaderProgram {
    program: glium::Program,
}

impl ShaderProgram {
    //---------------------------------------------------------------------------------------------------
    pub fn from_string<'a>(
        display: &glium::Display,
        vs_raw: &'a str,
        fs_raw: &'a str,
    ) -> Result<ShaderProgram, glium::ProgramCreationError> {
        match glium::Program::from_source(display, vs_raw, fs_raw, None) {
            Ok(program) => Ok(ShaderProgram { program }),
            Err(e) => {
                println!("[ShaderProgram] Could not create shader program: {}", e);
                Err(e)
            }
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn from_source<'a>(
        display: &glium::Display,
        vs_file: &'a str,
        fs_file: &'a str,
    ) -> Result<ShaderProgram, glium::ProgramCreationError> {
        
        let vs_contents = std::fs::read_to_string(vs_file)
            .expect(&format!("[ShaderProgram] Could not open vertex shader file '{}'", vs_file)[..]);

        let fs_contents = std::fs::read_to_string(fs_file)
            .expect(&format!("[ShaderProgram] Could not open fragment shader file '{}'", fs_file)[..]);

        ShaderProgram::from_string(display, &vs_contents, &fs_contents)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn program(&self) -> &glium::Program
    {
        &self.program
    }
}
