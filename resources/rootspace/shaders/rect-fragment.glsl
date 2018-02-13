#version 330 core

uniform sampler2D diff_tex;
uniform sampler2D norm_tex;

in vec2 f_tex_coord;

out vec4 color;

void main() {
    color = texture(diff_tex, f_tex_coord);
}
