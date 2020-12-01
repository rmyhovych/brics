#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(location = 0) out vec3 v_out_Color;
layout(location = 1) out vec3 v_out_Norm;
layout(location = 2) out vec3 v_out_FragPos;
layout(location = 3) out vec3 v_out_CamPosition;

layout(set = 0, binding = 0) uniform Camera {
    mat4 m_in_PV;
    vec3 v_in_CamPosition;
};

layout(set = 0, binding = 1) uniform ObjectState {
    mat4 m_in_Model;
    vec3 v_in_Color;
};

void main() {
    v_out_Color = v_in_Color;
    v_out_Norm = transpose(inverse(mat3(m_in_Model))) * a_Norm;
    v_out_CamPosition = v_in_CamPosition;

    vec4 pos = m_in_Model * vec4(a_Pos, 1.0);
    v_out_FragPos = vec3(pos);
    gl_Position = m_in_PV * pos;
}
