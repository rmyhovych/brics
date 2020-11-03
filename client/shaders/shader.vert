#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(location = 0) out vec3 v_out_Color;
layout(location = 1) out vec3 v_out_Norm;

layout(set = 0, binding = 0) uniform Camera {
    mat4 m_in_Projection;
    mat4 m_in_View;
};

layout(set = 0, binding = 1) uniform Object {
    mat4 m_in_Rotation;
    mat4 m_in_Model;
    vec3 v_in_Color;
};

void main() {
    v_out_Color = v_in_Color;
    v_out_Norm = vec3(m_in_Rotation * vec4(a_Norm, 1.0));
    gl_Position = m_in_Projection * m_in_View * m_in_Model * vec4(a_Pos, 1.0);
}
