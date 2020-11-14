#version 450

layout(location = 0) in vec3 v_Color;
layout(location = 1) in vec3 v_in_Norm;

layout(location = 0) out vec4 o_Color;

void main() {
    vec3 light_dir = vec3(-0.4729, 0.7881, -0.3941);

    float intensity = max(dot(v_in_Norm, light_dir), 0.0);

    o_Color = vec4(intensity * v_Color, 1.0);
}
