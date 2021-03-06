#version 330 core

in vec2 uv;
in vec3 frag_col;

out vec4 col;

uniform float radius;
uniform float alpha;

// sdf of a rectangle of half-dimensions dim, centered at p0
float sdf(vec2 p0, vec2 dim, vec2 p) {
  return length(max(vec2(0, 0), abs(p - p0) - dim));
}

void main() {
  vec2 dim = vec2(0.5, 0.5);
  if (radius > 0.0) {
    float val = radius - sdf(dim, dim - radius * vec2(1.0, 1.0), uv);
    col = vec4(frag_col, smoothstep(-0.005, 0.005, val * alpha));
  }
  else {
    col = vec4(frag_col, alpha);
  }
}
