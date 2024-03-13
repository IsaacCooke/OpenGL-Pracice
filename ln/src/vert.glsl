#version 330 core

layout (location = 0) in vec3 Position;

void main()
{
  gl_position = vec4(Position, 1.0f);
}
