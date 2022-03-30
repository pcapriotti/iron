#version 330 core

in vec2 uv;
out vec4 col;
uniform sampler2D t;

void main() {
  float val = texture(t, uv).r;
  col = vec4(val, val, val, 1.0);
}
