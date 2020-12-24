#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(set = 0, binding = 0) uniform u_Camera {
    mat4 m_in_PV;
    vec3 v_in_CamPosition;
};


layout(set = 0, binding = 1) uniform u_ObjectState {
    mat4 m_in_Model;
    vec3 v_in_Color;
};

void main() {
    gl_Position = m_in_PV * m_in_Model * vec4(a_Pos, 1.0);
}
