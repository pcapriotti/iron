#version 430 core
layout (location = 0) in vec2 p; // vertex coordinates in unit square
layout (location = 1) in ivec4 cell_rect; // rect for the whole cell in pixels
layout (location = 2) in int glyph; // index of the glyph in the atlas

struct glyph_info_t {
  vec4 uv_rect;
  vec4 rect;
};

layout(std430, binding = 0) buffer atlas_t {
  glyph_info_t info[];
} atlas;

uniform ivec4 viewport;
uniform vec2 scale;
uniform uvec2 resolution;
out vec2 uv;
flat out vec4 uv_rect;

void main() {
  glyph_info_t info = atlas.info[glyph];
  /* uv = info.uv_rect.xy + info.uv_rect.zw * uv0; */

  /* vec2 pos = vec2(cell_rect.x + info.rect.x, */
  /*   cell_rect.y + cell_rect.w - info.rect.y - info.rect.w); */
  /* pos += info.rect.zw * p; */

  // position of cell rect vertex in pixel coordinates
  vec2 pos = cell_rect.xy + cell_rect.zw * p;

  // unit space -> texture space mapping
  // f(x) = alpha x + beta
  vec2 alpha = info.uv_rect.zw / info.rect.zw;
  vec2 beta = info.uv_rect.xy - alpha * info.rect.xy;

  /* uv = info.uv_rect.xy + info.uv_rect.zw * uv0; */
  uv = beta + alpha * vec2(p.x, p.y);
  uv_rect = info.uv_rect;

  gl_Position = vec4(
    (pos - viewport.xy) * 2 / viewport.zw - vec2(1.0, 1.0),
    0.0, 1.0);
}
