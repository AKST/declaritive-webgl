#version 300 es

in vec2 position;

out vec4 v_color;

uniform vec2 resolution;

void main() {
  // convert the position from pixels to 0.0 to 1.0
  vec2 zeroToOne = position / resolution;

  // convert from 0->1 to 0->2 to -1->+1
  vec2 clipSpace = (zeroToOne * 2.0) - 1.0;

  // flip the y axis, so (0, 0) is the top left
  vec2 clipSpace2d = clipSpace * vec2(1, -1);

  gl_Position = vec4(clipSpace2d, 0, 1);
  v_color = gl_Position * 0.5 + 0.5;
}
