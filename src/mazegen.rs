//
//   #.            .#  "###
//   ###.        .###    "#
//   ####  ####  ####
//   ####  ####  ####
//   ####  ####  ####  
//   ####  ####  ####  #.
//   ####  ####  ####  ###.
//
//  ###########  ####  #####
//               "###
//                 "#
//
// MazeGen (Legacy).
//

extern crate rand;

use rand::Rng;

pub struct MazeCell2 {
    open_dir: u8,
    visited: bool,
    listed: bool,
}

impl MazeCell2 {
    
    pub fn new() -> MazeCell2 {
        
        return MazeCell2 {
            open_dir: 0,
            visited: false,
            listed: false,
        };

    }

    pub fn visit(&mut self) {
        self.visited = true;
    } 

    pub fn list(&mut self) {
        self.listed = true;
    }

    pub fn unlist(&mut self) {
        self.listed = false;
    }

    pub fn open(&mut self, dir: u8) {
        self.open_dir |= 1 << dir;
    }

    pub fn is_visited(&self) -> bool {
        return self.visited;
    }

    pub fn is_listed(&self) -> bool {
        return self.listed;
    }

    pub fn val(&self) -> u8 {
        return self.open_dir;
    }

    pub fn is_open(&self, dir:u8) -> bool {
        return self.open_dir >> dir & 1 == 1;
    }

}

pub struct Maze2 {
    generated: bool,
    size: (usize, usize),
    data: Vec<Vec<MazeCell2>>
}

impl Maze2 {
    
    pub fn empty(
        size_x: usize, 
        size_y: usize
    ) -> Maze2 {
    
        let mut maze = Maze2 {
            generated: false,
            size: (size_x, size_y),
            data: Vec::new()
        };
        for i in 0..size_x {
            maze.data.push(Vec::new());
            for _ in 0..size_y {
                maze.data[i].push(MazeCell2::new());
            }
        }
        return maze;

    }

    pub fn get_cell_val(
        &self,
        x: usize,
        y: usize
    ) -> u8 {
        return self.data[x][y].val();
    }

    pub fn is_cell_open(
        &self,
        x: usize,
        y: usize,
        dir: u8
    ) -> bool {
        return self.data[x][y].is_open(dir);
    }

    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }

    pub fn borrow_data(&self) -> &Vec<Vec<MazeCell2>> {
        return &self.data;
    }

    pub fn generate(
        &mut self 
    ) {

        if self.generated {
            panic!("Maze already generated!")
        }

        // Initialize generator
        let mut rng = rand::thread_rng();
        let mut to_be_generated: Vec<(usize, usize)> = Vec::new();
        
        // Starting cell
        let mut current_pos: (usize, usize) = (
            rng.gen_range(0, self.size.0), 
            rng.gen_range(0, self.size.1)
        );
        self.data[current_pos.0][current_pos.1].visit();

        // List all adjacent cells
        if current_pos.0 > 0 {
            to_be_generated.push((current_pos.0 - 1, current_pos.1));
            self.data[current_pos.0 - 1][current_pos.1].list();
        }
        if current_pos.0 < self.size.0 - 1 {
            to_be_generated.push((current_pos.0 + 1, current_pos.1));
            self.data[current_pos.0 + 1][current_pos.1].list();
        }
        if current_pos.1 > 0 {
            to_be_generated.push((current_pos.0, current_pos.1 - 1));
            self.data[current_pos.0][current_pos.1 - 1].list();
        }
        if current_pos.1 < self.size.1 - 1 {
            to_be_generated.push((current_pos.0, current_pos.1 + 1));
            self.data[current_pos.0][current_pos.1 + 1].list();
        }

        // Generate cells randomly
        while to_be_generated.len() > 0 {
            // Select random cell from to be generated list
            let rand_cell = rng.gen_range(0, to_be_generated.len());
            current_pos = to_be_generated[rand_cell];

            // Check possible directions
            let mut rand_dirs: Vec<u8> = Vec::new();
            if current_pos.0 < self.size.0 - 1 {
                if self.data[current_pos.0 + 1][current_pos.1].is_visited() {
                    rand_dirs.push(0); 
                } else if !self.data[current_pos.0 + 1][current_pos.1].is_listed() {
                    to_be_generated.push((current_pos.0 + 1, current_pos.1));
                    self.data[current_pos.0 + 1][current_pos.1].list();
                }
            }
            if current_pos.0 > 0 {
                if self.data[current_pos.0 - 1][current_pos.1].is_visited() {
                    rand_dirs.push(1); 
                } else if !self.data[current_pos.0 - 1][current_pos.1].is_listed() {
                    to_be_generated.push((current_pos.0 - 1, current_pos.1));
                    self.data[current_pos.0 - 1][current_pos.1].list();
                }
            }
            if current_pos.1 < self.size.1 - 1 {
                if self.data[current_pos.0][current_pos.1 + 1].is_visited() {
                    rand_dirs.push(2); 
                } else if !self.data[current_pos.0][current_pos.1 + 1].is_listed() {
                    to_be_generated.push((current_pos.0, current_pos.1 + 1));
                    self.data[current_pos.0][current_pos.1 + 1].list();
                }
            }
            if current_pos.1 > 0 {
                if self.data[current_pos.0][current_pos.1 - 1].is_visited() {
                    rand_dirs.push(3); 
                } else if !self.data[current_pos.0][current_pos.1 - 1].is_listed() {
                    to_be_generated.push((current_pos.0, current_pos.1 - 1));
                    self.data[current_pos.0][current_pos.1 - 1].list();
                }
            }

            // Select random directions and open passage
            let rand_dir: u8 = rand_dirs[rng.gen_range(0, rand_dirs.len())];
            self.data[current_pos.0][current_pos.1].open(rand_dir);
            match rand_dir {
                0 => {
                    self.data[current_pos.0 + 1][current_pos.1].open(1);
                }
                1 => {
                    self.data[current_pos.0 - 1][current_pos.1].open(0);
                }
                2 => {
                    self.data[current_pos.0][current_pos.1 + 1].open(3);
                }
                3 => {
                    self.data[current_pos.0][current_pos.1 - 1].open(2);
                }
                _ => {}
            }

            // Mark cell as visisted
            self.data[current_pos.0][current_pos.1].visit();
            // Unlist cell
            self.data[current_pos.0][current_pos.1].unlist();
            to_be_generated.swap_remove(rand_cell);
        }

        self.generated = true;

    }

    pub fn draw_text(&self) {

        for row in self.data.iter() {
            for cell in (*row).iter() {
                print!("██");
                if (*cell).is_open(2) {
                    print!("██");
                } else {
                    print!("  ");
                }
            }
            print!("\n");
            for cell in (*row).iter() {
                if (*cell).is_open(0) {
                    print!("██");
                } else {
                    print!("  ");
                }
                print!("  ");
            }
            print!("\n");
        }
    
    }

}
