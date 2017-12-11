#version 330 core

uniform mat4 pvm_matrix;

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

void main() {
    vec4 v_position = pvm_matrix * vec4(position, 1.0);

    gl_Position = v_position;
}
