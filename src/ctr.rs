extern crate glfw;
extern crate nalgebra_glm as glm;

pub fn get_cursor_offset(
    window: &mut glfw::Window
) -> (f64, f64) {

    let (width, height): (i32, i32) =  window.get_size();
    let (mouse_x, mouse_y): (f64, f64) = window.get_cursor_pos();
    window.set_cursor_pos(width as f64 / 2.0, height as f64 / 2.0);
    return (
        mouse_x - (width as f64) / 2.0,
        mouse_y - (height as f64) / 2.0
    );

}

pub fn get_wasd(
    window: &glfw::Window    
) -> (f32, f32) {

    let mut v: (f32, f32) = (0.0, 0.0);
     
    if window.get_key(glfw::Key::W) == glfw::Action::Press {
        v.0 += 1.0; 
    }
    if window.get_key(glfw::Key::S) == glfw::Action::Press {
        v.0 -= 1.0;
    }
    if window.get_key(glfw::Key::A) == glfw::Action::Press {
        v.1 -= 1.0;
    }
    if window.get_key(glfw::Key::D) == glfw::Action::Press {
        v.1 += 1.0;
    }

    return normalize(v);

}

fn normalize(
    v: (f32, f32)    
) -> (f32, f32) {

    let mag: f32 = (v.0 * v.0 + v.1 * v.1).sqrt();
    if mag == 0.0 {
        return (0.0, 0.0);
    }
    return (v.0 / mag, v.1 / mag);

}
