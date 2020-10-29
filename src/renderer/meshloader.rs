extern crate gl;
extern crate scan_fmt;

use gl::types::*;
use std::io::BufRead;

pub trait OBJLoader {

    fn load_obj<P: AsRef<std::path::Path>>(
        &mut self,
        obj_path: P,
        mtl_path: P
    ) -> Result<(), std::io::Error>;

}

#[derive(Copy, Clone)]
pub struct MTLMaterial {
    pub ka: (f32, f32, f32),
    pub kd: (f32, f32, f32),
    pub ks: (f32, f32, f32),
    pub ns: f32,
}

impl MTLMaterial {
    pub fn new() -> MTLMaterial {

        return MTLMaterial {
            ka: (0.0, 0.0, 0.0),
            kd: (0.0, 0.0, 0.0),
            ks: (0.0, 0.0, 0.0),
            ns: 0.0
        }

    }
}

pub struct MTLFileData {
    pub materials:  std::collections::HashMap::<String, MTLMaterial>
}

impl MTLFileData {

    pub fn read_mtl_file<P: AsRef<std::path::Path>>(
        mtl_path: P,
    ) -> Result<MTLFileData, std::io::Error> {
        
        let mut materials = std::collections::HashMap::new();
        let mut current_mat = String::from("");
        let mtl_file = std::fs::File::open(mtl_path)?;
        let mut mtl_lines = std::io::BufReader::new(mtl_file).lines();
        while let Some(Ok(line)) = mtl_lines.next() {
            if let Ok(mat_name) = scan_fmt!(&line, "newmtl {}", String) {
                current_mat = mat_name;
                materials.insert(String::from(&current_mat), MTLMaterial::new());
            } else if let Ok(val) = scan_fmt!(&line, "Ns {}", f32) {
                if let Some(mat) = materials.get_mut(&current_mat) {
                    mat.ns = val;
                }
            } else if let Ok((r, g, b)) = scan_fmt!(&line, "Ka {} {} {}", f32, f32, f32) {
                if let Some(mat) = materials.get_mut(&current_mat) {
                    mat.ka = (r, g, b);
                }
            } else if let Ok((r, g, b)) = scan_fmt!(&line, "Kd {} {} {}", f32, f32, f32) {
                if let Some(mat) = materials.get_mut(&current_mat) {
                    mat.kd = (r, g, b);
                }
            } else if let Ok((r, g, b)) = scan_fmt!(&line, "Ks {} {} {}", f32, f32, f32) {
                if let Some(mat) = materials.get_mut(&current_mat) {
                    mat.ks = (r, g, b);
                }
            }
        }

        return Ok(MTLFileData {
            materials:  materials
        });

    }

}

pub struct OBJFileData {
    pub buf_pos:                Vec<gl::types::GLfloat>,
    pub buf_tex:                Vec<gl::types::GLfloat>,
    pub buf_norm:               Vec<gl::types::GLfloat>,
    pub buf_faces_pos_indices:  Vec<gl::types::GLuint>,
    pub buf_faces_tex_indices:  Vec<gl::types::GLuint>,
    pub buf_faces_norm_indices: Vec<gl::types::GLuint>,
    pub buf_faces_materials:    Vec<String>
}

impl OBJFileData {

    pub fn read_obj_file<P: AsRef<std::path::Path>>(
        obj_path: P,
    ) -> Result<OBJFileData, std::io::Error> {

        let mut odata = OBJFileData {
            buf_pos:                Vec::new(),
            buf_tex:                Vec::new(),
            buf_norm:               Vec::new(),
            buf_faces_pos_indices:  Vec::new(),
            buf_faces_tex_indices:  Vec::new(),
            buf_faces_norm_indices: Vec::new(),
            buf_faces_materials:    Vec::new()
        };
        let mut current_mat = String::from("");
        let obj_file = std::fs::File::open(obj_path)?;
        let mut obj_lines = std::io::BufReader::new(obj_file).lines();
        while let Some(Ok(line)) = obj_lines.next() {
            if let Ok((x, y, z)) = scan_fmt!(&line, "v {} {} {}", GLfloat, GLfloat, GLfloat) {
                odata.buf_pos.push(x);
                odata.buf_pos.push(y);
                odata.buf_pos.push(z);
            } else if let Ok((u, v)) = scan_fmt!(&line, "vt {} {}", GLfloat, GLfloat) {
                odata.buf_tex.push(u);
                odata.buf_tex.push(v);
            } else if let Ok((x, y, z)) = scan_fmt!(&line, "vn {} {} {}", GLfloat, GLfloat, GLfloat) {
                odata.buf_norm.push(x);
                odata.buf_norm.push(y);
                odata.buf_norm.push(z);
            } else if let Ok(mat_name) = scan_fmt!(&line, "usemtl {}", String) {
                current_mat = mat_name;
            } else if let Ok((v1, vt1, vn1, v2, vt2, vn2, v3, vt3, vn3)) = scan_fmt!(
                &line,
                "f {}/{}/{} {}/{}/{} {}/{}/{}",
                GLuint, GLuint, GLuint,
                GLuint, GLuint, GLuint,
                GLuint, GLuint, GLuint
            ) {
                odata.buf_faces_pos_indices.push(v1 - 1);
                odata.buf_faces_tex_indices.push(vt1 - 1);
                odata.buf_faces_norm_indices.push(vn1 - 1);
                odata.buf_faces_pos_indices.push(v2 - 1);
                odata.buf_faces_tex_indices.push(vt2 - 1);
                odata.buf_faces_norm_indices.push(vn2 - 1);
                odata.buf_faces_pos_indices.push(v3 - 1);
                odata.buf_faces_tex_indices.push(vt3 - 1);
                odata.buf_faces_norm_indices.push(vn3 - 1);
                odata.buf_faces_materials.push(String::from(&current_mat));
                odata.buf_faces_materials.push(String::from(&current_mat));
                odata.buf_faces_materials.push(String::from(&current_mat));
            }
        }

        return Ok(odata);

    }

}

