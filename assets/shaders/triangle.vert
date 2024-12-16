#version 300 es

in vec4 position;

precision highp float;
out vec4 v_color;

void main() {
    gl_Position = position;
    v_color = vec4(position.x*1.4 + 0.3, (-position.x-position.y) * 0.7 + 0.2, position.y + 0.5, 1);
}