pub mod shader;
pub mod mesh;
pub mod program;
pub mod meshloader;
pub mod glmesh;

pub use shader::*;
pub use program::*;

macro_rules! raw_vertex {

    ( pos: $x:expr, $y:expr, $z:expr; col: $r:expr, $g:expr, $b:expr ) => {
        renderer::mesh::Vertex::new(
            renderer::mesh::Triplef32::new($x, $y, $z),
            renderer::mesh::Triplef32::new($r, $g, $b)
        )
    }

}

macro_rules! raw_triangle {

    (
        pos: $x1:expr, $y1:expr, $z1:expr;
        col: $r1:expr, $g1:expr, $b1:expr;
        pos: $x2:expr, $y2:expr, $z2:expr;
        col: $r2:expr, $g2:expr, $b2:expr;
        pos: $x3:expr, $y3:expr, $z3:expr;
        col: $r3:expr, $g3:expr, $b3:expr 
    )=> {
        renderer::mesh::Triangle::new(
            raw_vertex!(
                pos: $x1, $y1, $z1;
                col: $r1, $g1, $b1
            ),
            raw_vertex!(
                pos: $x2, $y2, $z2;
                col: $r2, $g2, $b2
            ),
            raw_vertex!(
                pos: $x3, $y3, $z3;
                col: $r3, $g3, $b3
            )
        )
    }

}

