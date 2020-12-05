#version 450

layout(location = 0) in vec3 v_in_Color;
layout(location = 1) in vec3 v_in_Norm;
layout(location = 2) in vec3 v_in_FragPos;
layout(location = 3) in vec3 v_in_CamPosition;

layout(location = 0) out vec4 o_Color;

layout(set = 0, binding = 2) uniform u_Light {
    vec3 v_in_LightPosition;
    float f_in_LightIntensity;
    vec3 v_in_LightColor;
};

void main() {
    vec3 to_light = v_in_LightPosition - v_in_FragPos;
    float distance_from_light = length(to_light);

    vec3 light_dir = to_light / distance_from_light;
    vec3 view_dir = normalize(v_in_CamPosition - v_in_FragPos);
    vec3 reflect_dir = reflect(-light_dir, v_in_Norm);

    vec3 specular = 0.5 * pow(max(dot(view_dir, reflect_dir), 0.0), 16) * v_in_LightColor;
    vec3 diffuse = max(dot(v_in_Norm, light_dir), 0.0) * v_in_LightColor;

    vec3 result = pow(f_in_LightIntensity, -distance_from_light) * (diffuse + specular) * v_in_Color;

    o_Color = vec4(result, 1.0);
}
