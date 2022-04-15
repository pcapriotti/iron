#version 330 core

in vec2 uv;
flat in vec4 uv_rect;
out vec4 col;
uniform sampler2D t;

void main() {
  float val = 0.0;
  if (uv.x >= uv_rect.x && uv.y >= uv_rect.y &&
    uv.x <= uv_rect.x + uv_rect.z &&
    uv.y <= uv_rect.y + uv_rect.w) {
    val = texture(t, uv).r;
  }
  vec4 bg = vec4(0.9, 0.9, 0.9, 1.0);
  vec4 fg = vec4(0.1, 0.1, 0.1, 1.0);
  col = mix(bg, fg, val);
}
