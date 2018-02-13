#version 330 core

uniform sampler2D font_cache;
uniform vec3 font_color;

in vec2 f_tex_coord;

out vec4 color;

void main() {
    vec4 text_data = texture(font_cache, f_tex_coord);
    float alpha = text_data.r;
    float color_factor = text_data.a;
    color = vec4(font_color * color_factor * alpha, alpha);
}
