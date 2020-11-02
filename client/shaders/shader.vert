#version 450

layout(location = 0) in vec3 a_Pos;

layout(location = 0) out vec3 v_Color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 m_projection;
    mat4 m_view;
};

layout(set = 0, binding = 1) uniform Object {
    mat4 m_model;
    vec3 m_color;
};

void main() {
    v_Color = m_color;
    gl_Position = m_projection * m_view * m_model * vec4(a_Pos, 1.0);
}
