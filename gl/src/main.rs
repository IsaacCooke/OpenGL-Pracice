extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{Action, Context, Key};

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            800,
            600,
            "OpenGL Image Rendering",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);

    // Load the image (you'll need to replace this with your actual image loading code)
    // For simplicity, let's assume the image is RGBA format
    let image_width = 512;
    let image_height = 512;
    let image_data: Vec<u8> = vec![255; image_width * image_height * 4]; // Fill with white for now

    // Set up OpenGL
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Viewport(0, 0, image_width as i32, image_height as i32);
    }

    // Main rendering loop
    while !window.should_close() {
        // Clear the screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Render the image
        unsafe {
            gl::RasterPos2i(0, 0); // Set the raster position
            gl::DrawPixels(
                image_width as i32,
                image_height as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_data.as_ptr() as *const GLvoid,
            );
        }

        // Swap buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
