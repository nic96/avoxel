#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in uint Texture_Datum;

#ifdef STANDARDMATERIAL_NORMAL_MAP
layout(location = 3) in vec4 Vertex_Tangent;
#endif

layout(location = 0) out vec3 v_WorldPosition;
layout(location = 1) out vec3 v_WorldNormal;
layout(location = 2) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

#ifdef STANDARDMATERIAL_NORMAL_MAP
layout(location = 3) out vec4 v_WorldTangent;
#endif

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 1, binding = 1) uniform FogSettings {
    vec4 FogColor;
    float FogNear;
    float FogFar;
};
layout(location = 4) out float v_Layer;
layout(location = 5) out float v_FogAmount;

void main() {
    vec4 world_position = Model * vec4(Vertex_Position, 1.0);
    v_WorldPosition = world_position.xyz;
    v_WorldNormal = mat3(Model) * Vertex_Normal;
    v_Uv = vec2(Texture_Datum >> 12u & 0x1u, Texture_Datum >> 13u & 0x1u);
    #ifdef STANDARDMATERIAL_NORMAL_MAP
    v_WorldTangent = vec4(mat3(Model) * Vertex_Tangent.xyz, Vertex_Tangent.w);
    #endif
    gl_Position = ViewProj * world_position;
    v_Layer = Texture_Datum & 0xFFFu;
    v_FogAmount = smoothstep(FogNear, FogFar, length(gl_Position.xyz));
}
