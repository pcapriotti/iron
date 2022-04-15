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
out vec2 uv;

void main() {
  glyph_info_t info = atlas.info[glyph];

  vec4 rect = vec4(info.rect.xy * cell_rect.zw, info.rect.zw * cell_rect.zw);
  vec2 pos = cell_rect.xy + rect.xy + rect.zw * p;
  uv = info.uv_rect.xy + info.uv_rect.zw * p;

  gl_Position = vec4(
    (pos - viewport.xy) * 2 / viewport.zw - vec2(1.0, 1.0),
    0.0, 1.0);
}
