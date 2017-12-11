#version 330 core

const vec3 specular_color = vec3(0.3 0.15, 0.1);

out vec4 color;

void main() {
    color = vec4(specular_color, 1.0);
}
