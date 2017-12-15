#version 330 core

uniform mat4 pvm_matrix;

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

out vec2 f_tex_coord;

void main() {
    f_tex_coord = tex_coord;
    gl_Position = pvm_matrix * vec4(position, 1.0);
}
