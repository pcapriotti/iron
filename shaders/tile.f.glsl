#version 330 core

in vec2 uv;
out vec4 col;
uniform sampler2D t;

void main() {
  float val = texture(t, uv).r;
  col = vec4(0.9, 0.9, 0.9, val);
}
