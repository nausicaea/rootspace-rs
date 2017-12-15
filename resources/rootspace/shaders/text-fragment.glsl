#version 330 core

uniform sampler2D font_cache;

in vec2 f_tex_coord;

out vec4 color;

void main() {
    float text = texture(font_cache, f_tex_coord).r;
    color = vec4(text, text, text, 1.0);
}
