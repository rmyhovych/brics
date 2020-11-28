#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(location = 0) out vec3 v_out_Color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 m_in_PV;
};

void main() {
    v_out_Color = vec3(1.0, 0.5, 0.5);

    gl_Position = m_in_PV * vec4(a_Pos, 1.0);
}
