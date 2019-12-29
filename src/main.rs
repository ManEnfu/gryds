// Main.rs

extern crate glfw;
extern crate gl;
extern crate nalgebra_glm as glm;

#[macro_use]
mod renderer;
mod cfile;
mod mazegen;
mod mazemesh;
mod ctr;
mod game;

use glfw::Context;
use renderer::mesh;
use core::cell::RefCell;

fn main() {

    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core
        )
    );

    // Create window
    let (mut window, events) = glfw.create_window(
        960, 
        540, 
        "GRYDS", 
        glfw::WindowMode::Windowed
    ).expect("Failed to create window.");

    // Initialize GL
    gl::load_with(|s| window.get_proc_address(s));
    
    // Setup Window
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_sticky_keys(true);
    window.set_key_polling(true);
    window.make_current();

    // Load shaders
    let vsh: renderer::shader::Shader = renderer::shader::Shader::source_path(
        std::path::Path::new("assets/shaders/triangle.vert"),
        gl::VERTEX_SHADER
    ).unwrap(); 

    let fsh: renderer::shader::Shader = renderer::shader::Shader::source_path(
        std::path::Path::new("assets/shaders/triangle.frag"),
        gl::FRAGMENT_SHADER
    ).unwrap(); 

    // Link shaders and use linked program
    let prog: renderer::shader::Program = 
        renderer::shader::Program::link_shaders(&[vsh, fsh])
        .unwrap();

    prog.use_program();

    // Default background color
    unsafe { 
        gl::ClearColor(0.6, 0.6, 0.6, 1.0); 
    }

    // Generate maze
    let mut maze: mazegen::Maze2 = mazegen::Maze2::empty(15, 15);
    maze.generate();
    
    // Create player
    let mut player = game::Player::new((7.5, 7.5, 0.5));

    // Create meshes
    let mut msdata: renderer::mesh::MeshData = renderer::mesh::MeshData::new();
    let maze_mesh = mazemesh::MazeMesh2::new(&maze, &mut msdata);
    let diamond_mesh = mazemesh::diamond(&mut msdata, 14.5, 14.5, 0.5, 0.2);
    
    maze.draw_text();

    // Setup meshes VAO
    unsafe {
        msdata.vertex_attrib_pointer();
    }

    // Enable some nice stuffs
    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    // Time tracking
    let mut old_time = glfw.get_time();
    let mut count: u32 = 0;

    // Main loop
    while !window.should_close() {

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let current_time = glfw.get_time();
        let dt: f32 = (current_time - old_time) as f32;

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }

        let (win_width, win_height) = window.get_size();
        
        unsafe {
            gl::Viewport(0, 0, win_width, win_height);
        }


        player.accept_input(&mut window, dt);
        player.update_in_maze(&maze, dt);

        let player_pos: (f32, f32, f32) = player.get_pos();
        let player_look: (f32, f32, f32) = player.get_look_vec();

        let mproj: glm::Mat4 = glm::perspective(
            (win_width as f32) / (win_height as f32), 
            0.63926, 
            0.1, 
            100.0
        );
        let mview: glm::Mat4 = glm::look_at(
            &glm::vec3(player_pos.0, player_pos.1, player_pos.2),
            &glm::vec3(
                player_pos.0 + player_look.0,
                player_pos.1 + player_look.1,
                player_pos.2 + player_look.2
            ),
            &glm::vec3(0.0, 0.0, 1.0)
        );
        let mmodel: glm::Mat4 = glm::diagonal4x4(&(glm::vec4(1.0, 1.0, 1.0, 1.0)));
        let mvp: glm::Mat4 = mproj * mview * mmodel;
        unsafe {
            let cs = std::ffi::CString::new("MVP").expect("NO!");
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(
                    prog.id(), 
                    cs.as_bytes_with_nul().as_ptr() as *const gl::types::GLchar
                ), 
                1, 
                gl::FALSE, 
                glm::value_ptr(&mvp).as_ptr() as *const gl::types::GLfloat
            );
        }
        
        maze_mesh.render();
        diamond_mesh.render();

        window.swap_buffers();
        old_time = current_time;
        count += 1;
        if count == 100 {
            count = 0;
            println!("{} {} {}", 1.0 / dt, player_pos.0, player_pos.1);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {

    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }

}
