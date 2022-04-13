#version 430 core
layout (location = 0) in vec2 p; // vertex coordinates in unit square
layout (location = 1) in vec2 uv0; // uv coordinates
layout (location = 2) in ivec4 cell_rect; // rect for the whole cell in pixels
layout (location = 3) in int glyph; // index of the glyph in the atlas

struct glyph_info_t {
  vec4 uv_rect;
  ivec4 rect;
};

layout(std430, binding = 0) buffer atlas_t {
  glyph_info_t info[];
} atlas;

uniform ivec4 viewport;
uniform vec2 scale;
uniform uvec2 resolution;
out vec2 uv;

void main() {
  glyph_info_t info = atlas.info[glyph];
  uv = info.uv_rect.xy + info.uv_rect.zw * uv0;

  /* vec2 pos = (cell_rect.xy + info.rect.xy); */
  vec2 pos = vec2(cell_rect.x + info.rect.x,
    cell_rect.y + cell_rect.w - info.rect.y - info.rect.w);
  pos += info.rect.zw * p;
  gl_Position = vec4(
    (pos - viewport.xy) * 2 / viewport.zw - vec2(1.0, 1.0),
    0.0, 1.0);
}
