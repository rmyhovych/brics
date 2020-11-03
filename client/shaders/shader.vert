#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(location = 0) out vec3 v_out_Color;
layout(location = 1) out vec3 v_out_Norm;

layout(set = 0, binding = 0) uniform Camera {
    mat4 m_in_Projection;
    mat4 m_in_View;
};

struct ObjectData {
    mat4 m_in_Rotation;
    mat4 m_in_Model;
    vec3 v_in_Color;
};

layout(set = 0, binding = 1) buffer Object {
    ObjectData s_objects[];
};

void main() {
    ObjectData object = s_objects[gl_InstanceIndex];

    v_out_Color = object.v_in_Color;
    v_out_Norm = vec3(object.m_in_Rotation * vec4(a_Norm, 1.0));

    gl_Position = m_in_Projection * m_in_View * object.m_in_Model * vec4(a_Pos, 1.0);
}
