#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(location = 0) out vec3 v_out_Color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 m_in_PV;
};

layout(set = 0, binding = 1) uniform Model {
    mat4 m_in_Model;
    vec3 v_in_Color;
}

void main() {
    v_out_Color = v_in_Color;

    gl_Position = m_in_PV * m_in_Model * vec4(a_Pos, 1.0);
}
