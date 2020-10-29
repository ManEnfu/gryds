extern crate gl;

use crate::cfile;

// shader::Shader
#[derive(Clone)]
pub struct Shader {
    shader_id   : gl::types::GLuint,
    shader_type : gl::types::GLenum
}

impl Shader {
    
    pub fn new(
        shader_id   : gl::types::GLuint,
        shader_type : gl::types::GLenum
    ) -> Shader {

        return Shader {
            shader_id   : shader_id,
            shader_type : shader_type
        };

    }

    pub fn id(
        &self
    ) -> gl::types::GLuint {
        return self.shader_id;
    }

}

impl Drop for Shader {

    fn drop(&mut self) {
        unsafe{
            gl::DeleteShader(self.shader_id);
        }
    }

}

#[derive(Clone)]
pub struct ShaderBuilder {
    shader_type:    gl::types::GLenum,
    shader_source:  std::ffi::CString
}

impl ShaderBuilder {

    pub fn new(
        shader_type:    gl::types::GLenum
    ) -> Result<ShaderBuilder, std::ffi::NulError> {

        return Ok(ShaderBuilder {
            shader_type:    shader_type,
            shader_source:  std::ffi::CString::new("")?
        });
    }
    
    pub fn source(
        &mut self,
        source: std::ffi::CString
    ) -> &mut Self {
        
        self.shader_source = source;
        return self;

    }
    
    pub fn source_path<P: AsRef<std::path::Path> + std::fmt::Debug>(
        &mut self,
        path:   P,
    ) -> Result<&mut Self, String> {
    
        self.shader_source = cfile::read_as_cstring(&path)
            .map_err(|e| format!("OOPS! {:?} {:?}", &path, e))?;
        return Ok(self);

    }
    
    pub fn compile(
        &self
    ) -> Result<Shader, String> { 

        let id: gl::types::GLuint = unsafe { gl::CreateShader(self.shader_type) };
        unsafe { 
            gl::ShaderSource(id, 1, &self.shader_source.as_ptr(), std::ptr::null());
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
            let log_msg = cstring_whitespaces(log_length as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    log_length, 
                    std::ptr::null_mut(), 
                    log_msg.as_ptr() as *mut gl::types::GLchar
                );
                gl::DeleteShader(id);
            }
            return Err(log_msg.to_string_lossy().into_owned());
        }

        return Ok(Shader::new(id, self.shader_type));

    }

}

// shader::cstring_whitespaces()
// Create new C++ style string with length defined.
pub fn cstring_whitespaces(length: usize) -> std::ffi::CString {
    
    let mut buffer: Vec<u8> = Vec::with_capacity(length + 1);
    buffer.extend([b' '].iter().cycle().take(length));
    unsafe {
        return std::ffi::CString::from_vec_unchecked(buffer);
    }

}
