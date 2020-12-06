#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(set = 0, binding = 0) uniform u_Camera {
    mat4 m_in_PV;
    vec3 v_in_CamPosition;
};

struct ObjectState {
    mat4 m_in_Model;
    vec3 v_in_Color;
};

layout(set = 0, binding = 1) buffer b_ObjectStates {
    ObjectState s_States[];
};


void main() {
    ObjectState current_state = s_States[gl_InstanceIndex];
    gl_Position = m_in_PV * current_state.m_in_Model * vec4(a_Pos, 1.0);
}
