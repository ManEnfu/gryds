extern crate gl;

// renderer::mesh::Triplef32
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Triplef32(pub f32, pub f32, pub f32);

impl Triplef32 {
    
    pub fn new(a: f32, b: f32, c: f32) -> Triplef32 {
        
        return Triplef32(a, b, c);

    }

}

// renderer::mesh::Vertex
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: Triplef32,
    pub col: Triplef32,
}

impl Vertex {
    
    pub fn new(
        pos: Triplef32,
        col: Triplef32,
    ) -> Vertex {
        
        return Vertex {
            pos: pos,
            col: col
        };

    }

}

// renderer::mesh::Triangle
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

impl Triangle {

    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Triangle {
    
        return Triangle(a, b, c);
    
    }

}

// renderer::mesh::MeshData
#[derive(Debug)]
pub struct MeshData {
    pub vbo_pos:    gl::types::GLuint,  // VBO for vertex positions
    pub vbo_col:    gl::types::GLuint,  // VBO for vertex colors
    pub vao:        gl::types::GLuint,  // VAO
    buf_pos:       Vec<Triplef32>,      // List of Triangles
    buf_col:       Vec<Triplef32>,      // List of Triangles
    start_indices:  Vec<i32>,   // List of start triangle indices for meshes.
    buffer_lengths: Vec<isize>  // List of tiangle count for meshes
}

impl MeshData {
   
    // renderer::mesh::MeshData::new()
    // Create new MeshData
    pub fn new() -> MeshData {
        let mut msdat: MeshData = MeshData {
            vbo_pos: 1,
            vbo_col: 1,
            vao: 1,
            buf_pos: Vec::new(),
            buf_col: Vec::new(),
            start_indices: Vec::new(),
            buffer_lengths: Vec::new()
        };
        unsafe {        
            gl::GenBuffers(1, &mut msdat.vbo_pos);
            gl::GenBuffers(1, &mut msdat.vbo_col);
            gl::GenVertexArrays(1, &mut msdat.vao);
        }
        return msdat;
    }

    pub unsafe fn buffer_data(&self) {

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_pos);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (self.buf_pos.len() * std::mem::size_of::<Triplef32>()) 
                as gl::types::GLsizeiptr,
            self.buf_pos.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_col);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (self.buf_col.len() * std::mem::size_of::<Triplef32>()) 
                as gl::types::GLsizeiptr,
            self.buf_col.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    
    }

    pub unsafe fn vertex_attrib_pointer(&self) {
    
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_pos);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer( 
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Triplef32>() as gl::types::GLint,
            std::ptr::null()
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_col);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer( 
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Triplef32>() as gl::types::GLint,
            std::ptr::null()
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    
    }

    pub fn add_mesh_triangles(
        &mut self,
        tris: &mut Vec<Triangle>
    ) -> (i32, isize) {

        let start_index: i32 = self.buf_pos.len() as i32;
        let buf_len: isize = 3 * tris.len() as isize;
        for triangle in tris {
            self.buf_pos.push(triangle.0.pos);
            self.buf_pos.push(triangle.1.pos);
            self.buf_pos.push(triangle.2.pos);
            self.buf_col.push(triangle.0.col);
            self.buf_col.push(triangle.1.col);
            self.buf_col.push(triangle.2.col);
        }
        self.start_indices.push(start_index);
        self.buffer_lengths.push(buf_len);
        unsafe {
            self.buffer_data();
        }
        return (start_index, buf_len);
    
    }

    pub fn draw_mesh(
        &self,
        buffer_index: usize
    ) {
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                self.start_indices[buffer_index] as gl::types::GLint,
                self.buffer_lengths[buffer_index] as gl::types::GLsizei
            );
        }
    }

}

impl Drop for MeshData {

    fn drop(&mut self) {
        unsafe{
            gl::DeleteBuffers(1, &self.vbo_pos);
            gl::DeleteBuffers(1, &self.vbo_col);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }

}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Mesh {
    pub vao:        gl::types::GLuint, // VAO
    start_index:    i32,    // Start index of triangle in buffer
    buffer_length:  isize   // Triangle count
}

impl Mesh {
    
    pub fn new(
        msdata: &mut MeshData,      // MeshData to store mesh to.
        tris:   &mut Vec<Triangle>  // List of Triangles
    ) -> Mesh {
        
        let (start_index, buf_len) = msdata.add_mesh_triangles(tris);
        return Mesh { 
            vao: msdata.vao,
            start_index: start_index,
            buffer_length: buf_len
        };

    }

    pub fn render(&self) { 

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                3 * self.start_index as gl::types::GLint,
                3 * self.buffer_length as gl::types::GLsizei
            );
            gl::BindVertexArray(0);
        }
    
    }

}
