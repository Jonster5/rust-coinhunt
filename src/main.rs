use gl;
use gl::types::*;
use glfw::{self, InitError, Key, PWindow, WindowEvent};
use rand::random;

fn main() -> Result<(), InitError> {
    println!("Hello, world! {}", random::<i32>());

    let mut glfw = glfw::init(glfw::fail_on_errors)?;

    let (mut window, events) = glfw
        .create_window(400, 400, "coin hunt", glfw::WindowMode::Windowed)
        .expect("Window should exist");
    let (width, height) = window.get_framebuffer_size();

    window.set_key_polling(true);
    gl::load_with(|ptr| glfw.get_proc_address_raw(ptr));

    unsafe {
        gl::Viewport(0, 0, width, height);
        clear_color(Color(0.0, 0.0, 0.0, 1.0));
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    Ok(())
}

const VERT_SHADER: &str = "#version 330 core

layout (location = 0) in vec3 position;
     
void main()
{
    gl_Position = vec4(position, 1.0);
    // gl_Position = vec4(position.xyz, 1.0);
    // gl_Position = vec4(position.x, position.y, position.z, 1.0);
}
";

fn handle_window_event(window: &mut PWindow, event: WindowEvent) {
    match event {
        WindowEvent::Key(key, _, _, _) => {
            if key == Key::Escape {
                window.set_should_close(true)
            }
        }
        _ => {}
    }
}

pub struct Color(f32, f32, f32, f32);

pub fn clear_color(c: Color) {
    unsafe { gl::ClearColor(c.0, c.1, c.2, c.3) }
}
