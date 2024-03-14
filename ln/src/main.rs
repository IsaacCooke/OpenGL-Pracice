extern crate gl;
extern crate glfw;
extern crate sdl2;

pub mod render_gl;

use glfw::Context;

use crate::render_gl::*;
use std::ffi::{c_void, CStr, CString};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw
        .create_window(900, 700, "Game", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.focus();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("vert.glsl")).unwrap())
            .unwrap();
    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("frag.glsl")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}
