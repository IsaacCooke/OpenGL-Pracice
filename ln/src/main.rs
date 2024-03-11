use gl::types::*;
use glam::{Vec2, Vec3};
use glfw::Context;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

const VERTEX_SHADER_SOURCE: &str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../shaders/fragment.glsl");

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Rectangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut vertex_array = 0;
    let mut vertex_buffer = 0;
    let mut texture = 0;

    unsafe {
        let vertices: [Vec3; 4] = [
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(0.5, 0.5, 0.0),
            Vec3::new(-0.5, 0.5, 0.0),
        ];

        let tex_coords: [Vec2; 4] = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];

        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);

        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        let vertices_data: Vec<_> = vertices
            .iter()
            .zip(tex_coords.iter())
            .flat_map(|(v, t)| vec![v.x, v.y, v.z, t.x, t.y])
            .collect();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let vertex_shader = compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
        let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);
        let program = link_program(vertex_shader, fragment_shader);

        gl::UseProgram(program);

        let texture_uniform =
            gl::GetUniformLocation(program, CString::new("texSampler").unwrap().as_ptr());

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        let img = image::open("../assets/logo.png").expect("Failed to load texture");
        let data = img.as_bytes();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            img.width() as GLint,
            img.height() as GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * mem::size_of::<GLfloat>()) as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * mem::size_of::<GLfloat>()) as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const GLfloat as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => (),
    }
}

fn compile_shader(shader_type: GLenum, source: &str) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "Shader compilation failed: {}",
                str::from_utf8(&buf).unwrap()
            );
        }
    }

    shader
}

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!("Program linking failed: {}", str::from_utf8(&buf).unwrap());
        }

        program
    }
}
