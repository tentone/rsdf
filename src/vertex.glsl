#version 140

in vec2 position;
in vec2 uv;

out vec2 v_uv;

uniform float time;

void main() {
    v_uv = uv;
    gl_Position = vec4(position, 0.0, 1.0);
}