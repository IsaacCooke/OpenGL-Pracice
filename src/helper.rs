use ogl33::{
    glBindBuffer, glBindVertexArray, glBufferData, glClearColor, glGenBuffers, glGenVertexArrays,
    glShaderSource, GLenum, GLuint, GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER, GL_FRAGMENT_SHADER,
    GL_VERTEX_SHADER,
};

pub fn clear_color(r: f32, b: f32, g: f32) {
    unsafe {
        glClearColor(r, g, b, 1.0);
    }
}

pub struct VertexArray(pub GLuint);

impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    pub fn bind(&self) {
        unsafe { glBindVertexArray(self.0) };
    }

    pub fn clear_binding() {
        unsafe { glBindVertexArray(0) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Holds vertex data
    Array = GL_ARRAY_BUFFER as isize,
    /// Holds indexes for verticies
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer(pub GLuint);

impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    pub fn bind(&self, ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, self.0) }
    }

    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer(ty as GLenum, 0) }
    }
}

pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        glBufferData(
            ty as GLenum,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}

pub enum ShaderType {
    Vertex = GL_VERTEX_SHADER as isize,
    Fragment = GL_FRAGMENT_SHADER as isize,
}

pub struct Shader(pub GLuint);
impl Shader {
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { glCreateShader(ty as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    pub fn set_source(&self, src: &str) {
        unsafe {
            glShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }
}
