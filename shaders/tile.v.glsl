#version 430 core
layout (location = 0) in vec2 p;

out vec2 uv;

void main() {
  uv = p;

  gl_Position = vec4(p, 0.5, 1.0);
}
