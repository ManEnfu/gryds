use super::shader::*;

#[derive(Clone)]
pub struct Program {
    program_id: gl::types::GLuint,
    shaders:    Vec<Shader>
}

impl Program {

    // shader::Program::link_shaders()
    // Link shaders from shaders slice.
    pub fn new(
        program_id: gl::types::GLuint,
        shaders:    Vec<Shader>
    ) -> Program {
        
        return Program {
            program_id: program_id,
            shaders:    shaders
        };

    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.program_id;
    }

    pub unsafe fn get_uniform_loc(
        &self,
        uniform: std::ffi::CString
    ) -> gl::types::GLint {
    
        return gl::GetUniformLocation(
            self.program_id,
            uniform.as_bytes_with_nul().as_ptr() as *const gl::types::GLchar
        );

    }

}

impl Drop for Program {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }

}

// shader::Program
#[derive(Clone)]
pub struct ProgramBuilder {
    shaders:    Vec<Shader>
}

impl ProgramBuilder {

    pub fn new() -> ProgramBuilder {

        return ProgramBuilder {
            shaders:    Vec::new()
        };

    }

    pub fn attach(
        &mut self,
        shader: Shader
    ) -> &mut Self {

        self.shaders.push(shader);
        return self;
    
    }

    pub fn link(&mut self) -> Result<Program, String> {

        let program_id: gl::types::GLuint = unsafe { gl::CreateProgram() };
        
        for shader in self.shaders.iter() {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }
        unsafe { gl::LinkProgram(program_id); }

        let mut link_success: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(
                program_id, 
                gl::LINK_STATUS, 
                &mut link_success
            );
        }
        if link_success == 0 {
            let mut log_length: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(
                    program_id, 
                    gl::INFO_LOG_LENGTH, 
                    &mut log_length
                );
            }
            let log_msg = cstring_whitespaces(log_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    log_length, 
                    std::ptr::null_mut(), 
                    log_msg.as_ptr() as *mut gl::types::GLchar
                );
                gl::DeleteProgram(program_id);
            }
            return Err(log_msg.to_string_lossy().into_owned());
        }

        let mut new_vec = Vec::<Shader>::new();
        for shader in self.shaders.drain(..) {
            new_vec.push(shader);
        }

        return Ok(Program::new(program_id, new_vec));

    }

}

