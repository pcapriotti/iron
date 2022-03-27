#version 330 core
layout (location = 0) in vec3 p;
layout (location = 2) in vec2 off;

void main() {
    gl_Position = vec4(p.x + off.x, p.y + off.y, p.z, 1.0);
}
