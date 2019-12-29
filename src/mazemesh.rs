use crate::renderer;
use crate::renderer::mesh::*;
use crate::mazegen::*;

pub fn cell2(
    msdata: &mut MeshData,
    x: usize,
    y: usize,
    dirs: u8,
    thiccness: f32
) -> Mesh {
    let x: f32 = x as f32;
    let y: f32 = y as f32;
    let a: f32 = 1.0 - thiccness;
    let b: f32 = 1.0 + thiccness;
    let c: f32 = thiccness;

    let mut tris: Vec<Triangle> = Vec::new();
    if dirs & 1 == 1 {
        tris.push(
            raw_triangle!(
                pos: a+x, a+y, a; col: 0.8, 0.8, 0.8;
                pos: a+x, a+y, c; col: 0.8, 0.8, 0.8;
                pos: b+x, a+y, c; col: 0.8, 0.8, 0.8
            )
        );
        tris.push(
            raw_triangle!(
                pos: a+x, a+y, a; col: 0.8, 0.8, 0.8;
                pos: b+x, a+y, c; col: 0.8, 0.8, 0.8;
                pos: b+x, a+y, a; col: 0.8, 0.8, 0.8
            )
        );
        tris.push(
            raw_triangle!(
                pos: b+x, c+y, a; col: 0.8, 0.8, 0.8;
                pos: b+x, c+y, c; col: 0.8, 0.8, 0.8;
                pos: a+x, c+y, c; col: 0.8, 0.8, 0.8
            )
        ); 
        tris.push(
            raw_triangle!(
                pos: b+x, c+y, a; col: 0.8, 0.8, 0.8;
                pos: a+x, c+y, c; col: 0.8, 0.8, 0.8;
                pos: a+x, c+y, a; col: 0.8, 0.8, 0.8
            )
        ); 
    } else {
        tris.push(
            raw_triangle!(
                pos: a+x, a+y, a; col: 0.7, 0.7, 0.7;
                pos: a+x, a+y, c; col: 0.7, 0.7, 0.7;
                pos: a+x, c+y, c; col: 0.7, 0.7, 0.7
            )
        );
        tris.push(
            raw_triangle!(
                pos: a+x, a+y, a; col: 0.7, 0.7, 0.7;
                pos: a+x, c+y, c; col: 0.7, 0.7, 0.7;
                pos: a+x, c+y, a; col: 0.7, 0.7, 0.7
            )
            );
    }

    if dirs >> 1 & 1 == 0 {
        tris.push(
            raw_triangle!(
                pos: c+x, c+y, a; col: 0.7, 0.7, 0.7;
                pos: c+x, c+y, c; col: 0.7, 0.7, 0.7;
                pos: c+x, a+y, c; col: 0.7, 0.7, 0.7
            )
        );
        tris.push(
            raw_triangle!(
                pos: c+x, c+y, a; col: 0.7, 0.7, 0.7;
                pos: c+x, a+y, c; col: 0.7, 0.7, 0.7;
                pos: c+x, a+y, a; col: 0.7, 0.7, 0.7
            )
        );
    }

    if dirs >> 2 & 1 == 1 {
        tris.push(
            raw_triangle!(
                pos: c+x, a+y, a; col: 0.7, 0.7, 0.7;
                pos: c+x, a+y, c; col: 0.7, 0.7, 0.7;
                pos: c+x, b+y, c; col: 0.7, 0.7, 0.7
            )
        );
        tris.push(
            raw_triangle!(
                pos: c+x, a+y, a; col: 0.7, 0.7, 0.7;
                pos: c+x, b+y, c; col: 0.7, 0.7, 0.7;
                pos: c+x, b+y, a; col: 0.7, 0.7, 0.7
            )
        );
        tris.push(
            raw_triangle!(
                pos: a+x, b+y, a; col: 0.7, 0.7, 0.7;
                pos: a+x, b+y, c; col: 0.7, 0.7, 0.7;
                pos: a+x, a+y, c; col: 0.7, 0.7, 0.7
            )
        ); 
        tris.push(
            raw_triangle!(
                pos: a+x, b+y, a; col: 0.7, 0.7, 0.7;
                pos: a+x, a+y, c; col: 0.7, 0.7, 0.7;
                pos: a+x, a+y, a; col: 0.7, 0.7, 0.7
            )
        ); 
    } else {
        tris.push(
            raw_triangle!(
                pos: c+x, a+y, a; col: 0.8, 0.8, 0.8;
                pos: c+x, a+y, c; col: 0.8, 0.8, 0.8;
                pos: a+x, a+y, c; col: 0.8, 0.8, 0.8
            )
        );
        tris.push(
            raw_triangle!(
                pos: c+x, a+y, a; col: 0.8, 0.8, 0.8;
                pos: a+x, a+y, c; col: 0.8, 0.8, 0.8;
                pos: a+x, a+y, a; col: 0.8, 0.8, 0.8
            )
        );
    }
    
    if dirs >> 3 & 1 == 0 {
        tris.push(
            raw_triangle!(
                pos: a+x, c+y, a; col: 0.8, 0.8, 0.8;
                pos: a+x, c+y, c; col: 0.8, 0.8, 0.8;
                pos: c+x, c+y, c; col: 0.8, 0.8, 0.8
            )
        );
        tris.push(
            raw_triangle!(
                pos: a+x, c+y, a; col: 0.8, 0.8, 0.8;
                pos: c+x, c+y, c; col: 0.8, 0.8, 0.8;
                pos: c+x, c+y, a; col: 0.8, 0.8, 0.8
            )
        );
    }

    return Mesh::new(msdata, &mut tris);

}

pub fn diamond(
    msdata: &mut MeshData,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    scale: f32
) -> Mesh {

    let x = pos_x;
    let y = pos_y;
    let z = pos_z;
    let s = scale;

    let mut tris: Vec<Triangle> = vec![
        raw_triangle!(
            pos: x, y, z+s; col: 0.0, 0.8, 0.8;
            pos: x+s, y, z; col: 0.0, 0.8, 0.8;
            pos: x, y+s, z; col: 0.0, 0.8, 0.8
        ),
        raw_triangle!(
            pos: x, y, z+s; col: 0.0, 0.7, 0.7;
            pos: x, y+s, z; col: 0.0, 0.7, 0.7;
            pos: x-s, y, z; col: 0.0, 0.7, 0.7
        ),
        raw_triangle!(
            pos: x, y, z+s; col: 0.0, 0.8, 0.8;
            pos: x-s, y, z; col: 0.0, 0.8, 0.8;
            pos: x, y-s, z; col: 0.0, 0.8, 0.8
        ),
        raw_triangle!(
            pos: x, y, z+s; col: 0.0, 0.7, 0.7;
            pos: x, y-s, z; col: 0.0, 0.7, 0.7;
            pos: x+s, y, z; col: 0.0, 0.7, 0.7
        ),
        raw_triangle!(
            pos: x, y+s, z; col: 0.0, 0.7, 0.7;
            pos: x+s, y, z; col: 0.0, 0.7, 0.7;
            pos: x, y, z-s; col: 0.0, 0.7, 0.7
        ),
        raw_triangle!(
            pos: x-s, y, z; col: 0.0, 0.6, 0.6;
            pos: x, y+s, z; col: 0.0, 0.6, 0.6;
            pos: x, y, z-s; col: 0.0, 0.6, 0.6
        ),
        raw_triangle!(
            pos: x, y-s, z; col: 0.0, 0.7, 0.7;
            pos: x-s, y, z; col: 0.0, 0.7, 0.7;
            pos: x, y, z-s; col: 0.0, 0.7, 0.7
        ),
        raw_triangle!(
            pos: x+s, y, z; col: 0.0, 0.6, 0.6;
            pos: x, y-s, z; col: 0.0, 0.6, 0.6;
            pos: x, y, z-s; col: 0.0, 0.6, 0.6
        )
    ];

    return Mesh::new(msdata, &mut tris);

}

pub struct MazeMesh2<'a> {
    maze: &'a Maze2,
    meshes: Vec<Mesh>
}

impl<'a> MazeMesh2<'a> {
    
    pub fn new(
        maze: &'a Maze2,
        msdata: &mut MeshData
    ) -> MazeMesh2<'a> {
    
        let mut mmesh: MazeMesh2 = MazeMesh2 {
            maze: maze,
            meshes: Vec::new()
        };
        let size = maze.get_size();
        for j in 0..size.1 {
            for i in 0..size.0 {
                mmesh.meshes.push(
                    cell2(msdata, i, j, maze.get_cell_val(i, j), 0.1)
                );
            }
        }
        return mmesh;
    
    }

    pub fn render(
        &self
    ) {
        
        for mesh in self.meshes.iter() {
            (*mesh).render();
        }

    }

}
