#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 0) out vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform Locals {
    mat4 u_Transform;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * a_Pos;
    gl_ClipDistance[0] = 1.0;
}
