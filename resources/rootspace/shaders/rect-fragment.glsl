#version 330 core

uniform sampler2D diff_tex;
uniform sampler2D norm_tex;

in vec2 f_tex_coord;

out vec4 color;

void main() {
    color = vec4(texture(diff_tex, f_tex_coord).rgb, 1.0);
}
