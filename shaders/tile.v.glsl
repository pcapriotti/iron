#version 330 core
layout (location = 0) in vec2 p;
layout (location = 1) in vec2 uv0;
layout (location = 2) in vec2 off;
layout (location = 3) in vec4 uv_rect;
layout (location = 4) in vec4 rect;

uniform vec2 scale;
uniform uvec2 resolution;
out vec2 uv;

void main() {
    vec2 pos = off + (vec2(rect.x, -rect.w - rect.y) + p * rect.zw) / resolution;
    uv = uv_rect.xy + uv0 * uv_rect.zw;
    gl_Position = vec4(pos * 2.0 - vec2(1.0, 1.0), 0.0, 1.0);
}
