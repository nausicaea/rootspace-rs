#version 330 core

uniform sampler2D font_cache;

in vec2 f_tex_coord;

out vec4 color;

void main() {
    float alpha_channel = texture(font_cache, f_tex_coord).r;
    vec3 color_channel = vec3(1.0 - alpha_channel);
    color = vec4(color_channel, 1.0 - alpha_channel);
}
