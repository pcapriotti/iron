#version 330 core
layout (location = 0) in vec2 p;
layout (location = 1) in ivec4 rect;
layout (location = 2) in vec3 colour;

out vec3 frag_col;
out vec2 uv;
uniform ivec4 viewport;

void main() {
  frag_col = colour;

  uv = p;
  vec2 pos = rect.xy + rect.zw * p;

  gl_Position = vec4(
    (pos - viewport.xy) * 2 / viewport.zw - vec2(1.0, 1.0),
    0.1, 1.0);
}
