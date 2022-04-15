#version 330 core

in vec3 frag_col;
out vec4 col;

void main() {
  col = vec4(frag_col, 1.0);
}
