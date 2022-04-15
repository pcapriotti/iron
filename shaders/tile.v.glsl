#version 430 core
layout (location = 0) in vec2 p;
layout (location = 1) in ivec4 rect;

out vec2 uv;
uniform ivec4 viewport;

void main() {
  uv = p;

  vec2 pos = rect.xy + rect.zw * p;

  gl_Position = vec4(
    (pos - viewport.xy) * 2 / viewport.zw - vec2(1.0, 1.0),
    0.0, 1.0);
}
