extern crate gl;

use crate::cfile;

// shader::Shader
#[derive(Clone)]
pub struct Shader(gl::types::GLuint);

impl Shader {
    
    // shader::Shader::source()
    // Load shader from source code
    // source: shader file content, not file path!
    // shader_type: gl::VERTEX_SHADER or gl::FRAGMENT_SHADER
    pub fn source(
        source:         &std::ffi::CStr, 
        shader_type:    gl::types::GLenum
    ) -> Result<Self, String> {

        let id: gl::types::GLuint;
        id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut compile_success: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_success);
        }
        if compile_success == 0 {
            let mut log_length: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
            }
            let log_msg: std::ffi::CString = cstring_whitespaces(log_length as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    log_length, 
                    std::ptr::null_mut(), 
                    log_msg.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(log_msg.to_string_lossy().into_owned());
        }

        return Ok(Shader(id));

    }

    pub fn source_path(
        path:           &std::path::Path,
        shader_type:    gl::types::GLenum
    ) -> Result<Shader, String> {
    
        let source = cfile::read_as_cstring(path)
            .map_err(|e| format!("OOPS! {} {:?}", path.display(), e))?;
        return Shader::source(&source, shader_type);

    }

}

impl Drop for Shader {

    fn drop(&mut self) {
        unsafe{
            gl::DeleteShader(self.0);
        }
    }

}

// shader::Program
#[derive(Clone)]
pub struct Program(gl::types::GLuint);

impl Program {

    // shader::Program::link_shaders()
    // Link shaders from shaders slice.
    pub fn link_shaders(
        shaders:    &[Shader]
    ) -> Result<Self, String> {
        
        let program_id: gl::types::GLuint = unsafe { gl::CreateProgram() };
        
        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.0);
            }
        }
        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut link_success: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut link_success);
        }
        if link_success == 0 {
            let mut log_length: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_length);
            }
            let log_msg: std::ffi::CString = cstring_whitespaces(log_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    log_length, 
                    std::ptr::null_mut(), 
                    log_msg.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(log_msg.to_string_lossy().into_owned());
        }

        return Ok(Program(program_id));

    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.0;
    }

}

impl Drop for Program {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }

}

// shader::cstring_whitespaces()
// Create new C++ style string with length defined.
fn cstring_whitespaces(length: usize) -> std::ffi::CString {
    
    let mut buffer: Vec<u8> = Vec::with_capacity(length + 1);
    buffer.extend([b' '].iter().cycle().take(length));
    unsafe {
        return std::ffi::CString::from_vec_unchecked(buffer);
    }

}
