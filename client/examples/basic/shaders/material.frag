#version 450

layout(location = 0) in vec3 v_in_Color;
layout(location = 1) in vec3 v_in_Norm;
layout(location = 2) in vec3 v_in_FragPos;
layout(location = 3) in vec3 v_in_CamPosition;
layout(location = 4) in mat4 m_in_PVLight;

layout(location = 0) out vec4 o_Color;

layout(set = 0, binding = 2) uniform u_Light {
    vec3 v_in_LightDirection;
    vec3 v_in_LightColor;
};

layout(set = 0, binding = 4) uniform texture2DArray t_Shadow;
layout(set = 0, binding = 5) uniform samplerShadow s_Shadow;

float fetch_shadow(vec4 homogeneous_coords) {
    if (homogeneous_coords.w <= 0.0) {
        return 1.0;
    }
    // compensate for the Y-flip difference between the NDC and texture coordinates
    const vec2 flip_correction = vec2(0.5, -0.5);
    // compute texture coordinates for shadow lookup
    vec4 light_local = vec4(
        homogeneous_coords.xy * flip_correction/homogeneous_coords.w + 0.5,
        0,
        homogeneous_coords.z / homogeneous_coords.w
    );
    // do the lookup, using HW PCF and comparison
    return texture(sampler2DArrayShadow(t_Shadow, s_Shadow), light_local);
}

void main() {
    vec3 light_dir = v_in_LightDirection;
    vec3 view_dir = normalize(v_in_CamPosition - v_in_FragPos);
    vec3 reflect_dir = reflect(-light_dir, v_in_Norm);

    vec3 specular = 0.5 * pow(max(dot(view_dir, reflect_dir), 0.0), 16) * v_in_LightColor;
    vec3 diffuse = max(dot(v_in_Norm, light_dir), 0.0) * v_in_LightColor;

    float shadow = 1.0; // fetch_shadow(m_in_PVLight * vec4(v_in_FragPos, 1.0));
    vec3 result = shadow * (diffuse + specular) * v_in_Color;

    o_Color = vec4(result, 1.0);
}
