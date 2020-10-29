extern crate gl;

use super::meshloader::*;
use gl::types::*;

unsafe fn handle_buffer_data<T>(
    vbo:    GLuint,
    buf:    &Vec<T>
) {

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (buf.len() * std::mem::size_of::<T>()) as GLsizeiptr,
        buf.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

}

unsafe fn handle_vertex_attrib_pointer(
    vao:        GLuint,
    vbo:        GLuint,
    components: GLint,
    attr_array: GLuint,
    attr_type:  GLenum
) {

    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::EnableVertexAttribArray(attr_array);
    gl::VertexAttribPointer(
        attr_array,
        components,
        attr_type,
        gl::FALSE,
        0 as GLint,
        std::ptr::null()
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

}



pub struct GLMesh {
    vbo_pos:    GLuint, 
    vbo_tex:    GLuint,
    vbo_norm:   GLuint,
    vbo_ka:     GLuint,
    vbo_kd:     GLuint,
    vbo_ks:     GLuint,
    vbo_ns:     GLuint,
    vao:        GLuint, 
    buf_pos:    Vec<GLfloat>,
    buf_norm:   Vec<GLfloat>,
    buf_tex:    Vec<GLfloat>,
    buf_ka:     Vec<GLfloat>,
    buf_kd:     Vec<GLfloat>,
    buf_ks:     Vec<GLfloat>,
    buf_ns:     Vec<GLfloat>
}

impl GLMesh {

    pub fn new() -> GLMesh {
        
        let mut new_mesh = GLMesh{
            vbo_pos:    0,
            vbo_tex:    0,
            vbo_norm:   0,
            vbo_ka:     0,
            vbo_kd:     0,
            vbo_ks:     0,
            vbo_ns:     0,
            vao:        0,
            buf_pos:    Vec::new(),
            buf_norm:    Vec::new(),
            buf_tex:    Vec::new(),
            buf_ka:    Vec::new(),
            buf_kd:    Vec::new(),
            buf_ks:    Vec::new(),
            buf_ns:    Vec::new()
        };
        unsafe {
            gl::GenBuffers(1, &mut new_mesh.vbo_pos);
            gl::GenBuffers(1, &mut new_mesh.vbo_norm);
            gl::GenBuffers(1, &mut new_mesh.vbo_tex);
            gl::GenBuffers(1, &mut new_mesh.vbo_ka);
            gl::GenBuffers(1, &mut new_mesh.vbo_kd);
            gl::GenBuffers(1, &mut new_mesh.vbo_ks);
            gl::GenBuffers(1, &mut new_mesh.vbo_ns);
            gl::GenVertexArrays(1, &mut new_mesh.vao);
        }
        return new_mesh;

    }

    pub fn buffer_data(&self) {
        
        unsafe {
            handle_buffer_data(self.vbo_pos, &self.buf_pos);
            handle_buffer_data(self.vbo_tex, &self.buf_tex);
            handle_buffer_data(self.vbo_norm, &self.buf_norm);
            handle_buffer_data(self.vbo_ka, &self.buf_ka);
            handle_buffer_data(self.vbo_kd, &self.buf_kd);
            handle_buffer_data(self.vbo_ks, &self.buf_ks);
            handle_buffer_data(self.vbo_ns, &self.buf_ns);
        }

    }

    pub fn vertex_attrib_pointer(&self) {
        
        unsafe {
            handle_vertex_attrib_pointer(self.vao, self.vbo_pos, 3, 0, gl::FLOAT);
            handle_vertex_attrib_pointer(self.vao, self.vbo_tex, 2, 1, gl::FLOAT);
            handle_vertex_attrib_pointer(self.vao, self.vbo_norm, 3, 2, gl::FLOAT);
            handle_vertex_attrib_pointer(self.vao, self.vbo_ka, 3, 3, gl::FLOAT);
            handle_vertex_attrib_pointer(self.vao, self.vbo_kd, 3, 4, gl::FLOAT);
            handle_vertex_attrib_pointer(self.vao, self.vbo_ks, 3, 5, gl::FLOAT);
            handle_vertex_attrib_pointer(self.vao, self.vbo_ns, 1, 6, gl::FLOAT);
        }

    }

    pub fn draw(&self) {

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0 as GLint,
                self.buf_ns.len() as GLsizei
            );
            gl::BindVertexArray(0);
        }

    }

}

impl OBJLoader for GLMesh {

    fn load_obj<P: AsRef<std::path::Path>>(
        &mut self,
        obj_path: P,
        mtl_path: P
    ) -> Result<(), std::io::Error> {

        let obj_data = OBJFileData::read_obj_file(obj_path)?;
        let mtl_data = MTLFileData::read_mtl_file(mtl_path)?;

        for vi in obj_data.buf_faces_pos_indices.iter() {
            self.buf_pos.push(obj_data.buf_pos[*vi as usize * 3]);
            self.buf_pos.push(obj_data.buf_pos[*vi as usize * 3 + 1]);
            self.buf_pos.push(obj_data.buf_pos[*vi as usize * 3 + 2]);
        }
        for vti in obj_data.buf_faces_tex_indices.iter() {
            self.buf_tex.push(obj_data.buf_tex[*vti as usize * 3]);
            self.buf_tex.push(obj_data.buf_tex[*vti as usize * 3 + 1]);
        }
        for vni in obj_data.buf_faces_norm_indices.iter() {
            self.buf_norm.push(obj_data.buf_norm[*vni as usize * 3]);
            self.buf_norm.push(obj_data.buf_norm[*vni as usize * 3 + 1]);
            self.buf_norm.push(obj_data.buf_norm[*vni as usize * 3 + 2]);
        }
        for mat_name in obj_data.buf_faces_materials.iter() {
            let mut mat = MTLMaterial::new();
            if let Some(m) = mtl_data.materials.get(mat_name) {
                mat = *m;
            }
            self.buf_ka.push(mat.ka.0);
            self.buf_ka.push(mat.ka.1);
            self.buf_ka.push(mat.ka.2);
            self.buf_kd.push(mat.kd.0);
            self.buf_kd.push(mat.kd.1);
            self.buf_kd.push(mat.kd.2);
            self.buf_ks.push(mat.ks.0);
            self.buf_ks.push(mat.ks.1);
            self.buf_ks.push(mat.ks.2);
            self.buf_ns.push(mat.ns);
        }

        return Ok(());

    }

}

impl Drop for GLMesh {

    fn drop(&mut self) {
        
        unsafe {
            gl::DeleteBuffers(1, &self.vbo_pos);
            gl::DeleteBuffers(1, &self.vbo_tex);
            gl::DeleteBuffers(1, &self.vbo_norm);
            gl::DeleteBuffers(1, &self.vbo_ka);
            gl::DeleteBuffers(1, &self.vbo_ks);
            gl::DeleteBuffers(1, &self.vbo_kd);
            gl::DeleteBuffers(1, &self.vbo_ns);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    
    }

}
