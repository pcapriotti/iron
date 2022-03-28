#version 330 core
layout (location = 0) in vec3 p;
layout (location = 2) in vec2 off;

uniform vec2 scale;

void main() {
    vec2 pos = (p.xy + off);
    gl_Position = vec4(pos.xy * scale, p.z, 1.0);
}
