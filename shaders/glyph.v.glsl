#version 330 core
layout (location = 0) in vec2 p;
layout (location = 1) in vec2 uv0;
layout (location = 2) in vec2 off;
layout (location = 3) in vec4 uv_rect;

uniform vec2 scale;
out vec2 uv;

void main() {
    vec2 pos = (p.xy + off);
    uv = (uv0 + uv_rect.xy) * uv_rect.zw;
    gl_Position = vec4(pos.xy * scale, 0.0, 1.0);
}
