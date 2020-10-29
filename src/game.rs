extern crate glfw;

use crate::mazegen::Maze2;
use crate::ctr;

pub struct Player {
    pos: (f32, f32, f32),
    vel: (f32, f32, f32),
    look_dir: (f32, f32),
    walk_speed: f32
}

impl Player {

    pub fn new (
        pos: (f32, f32, f32)
    ) -> Player {

        return Player {
            pos: pos,
            vel: (0.0, 0.0, 0.0),
            look_dir: (0.0, 0.0),
            walk_speed: 1.5
        }

    }

    pub fn get_pos(
        &self
    ) -> (f32, f32, f32) {
        return self.pos;
    }

    pub fn get_look_vec(
        &self    
    ) -> (f32, f32, f32) {
        return (
            self.look_dir.1.cos() * self.look_dir.0.cos(),
            self.look_dir.1.cos() * self.look_dir.0.sin(),
            self.look_dir.1.sin()
        );
    }

    pub fn accept_input(
        &mut self,
        window: &mut glfw::Window,
        dt: f32
    ) {

        let mut input_vel_x: f32 = 0.0;
        let mut input_vel_y: f32 = 0.0;
        
        let (in_x, in_y): (f32, f32) = ctr::get_wasd(window);
        let (walk_x, walk_y): (f32, f32) = (
            self.look_dir.0.cos(),
            self.look_dir.0.sin()
        );
        input_vel_x += self.walk_speed * (in_x * walk_x + in_y * walk_y);
        input_vel_y += self.walk_speed * (-in_y * walk_x + in_x * walk_y);
        
        self.vel.0 = (self.vel.0 * 8.0 + input_vel_x) / 9.0;
        self.vel.1 = (self.vel.1 * 8.0 + input_vel_y) / 9.0;
        
        let (mof_x, mof_y): (f64, f64) = 
            ctr::get_cursor_offset(window);
        
        self.look_dir.0 -= (mof_x as f32) * 0.05 * dt;
        self.look_dir.1 -= (mof_y as f32) * 0.05 * dt;

    }

    pub fn update_in_maze(
        &mut self,
        maze: &Maze2,
        dt: f32
    ) {

        // Precompute displacement
        let mut displacement_x: f32 = self.vel.0 * dt;
        let mut displacement_y: f32 = self.vel.1 * dt;

        // Check colission 
        let cell_x: usize = self.pos.0 as usize;
        let cell_y: usize = self.pos.1 as usize;
        let rel_x: f32 = self.pos.0.fract();
        let rel_y: f32 = self.pos.1.fract();
        
        if rel_x + displacement_x > 0.75 && self.vel.0 > 0.0 && (
            !maze.is_cell_open(cell_x, cell_y, 0) || 
            rel_y < 0.25 || 
            rel_y > 0.75
        ) {
            self.vel.0 = 0.0;
            displacement_x = 0.0;
        }
        if rel_x + displacement_x < 0.25 && self.vel.0 < 0.0 && (
            !maze.is_cell_open(cell_x, cell_y, 1) || 
            rel_y < 0.25 || 
            rel_y > 0.75
        ) {
            self.vel.0 = 0.0;
            displacement_x = 0.0;
        }
        if rel_y + displacement_y > 0.75 && self.vel.1 > 0.0 && (
            !maze.is_cell_open(cell_x, cell_y, 2) || 
            rel_x < 0.25 || 
            rel_x > 0.75
        ) {
            self.vel.1 = 0.0;
            displacement_y = 0.0;
        }
        if rel_y + displacement_y < 0.25 && self.vel.1 < 0.0 && (
            !maze.is_cell_open(cell_x, cell_y, 3) || 
            rel_x < 0.25 || 
            rel_x > 0.75
        ) {
            self.vel.1 = 0.0;
            displacement_y = 0.0;
        }
        self.pos.0 += displacement_x;
        self.pos.1 += displacement_y;

    }

}

pub struct Diamond {
    pos: (f32, f32, f32)
}

impl Diamond { 

}
