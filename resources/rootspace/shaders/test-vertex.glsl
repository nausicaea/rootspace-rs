#version 330 core

uniform mat4 pvm_matrix;

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

void main() {
    gl_Position = pvm_matrix * vec4(position, 1.0);
}
