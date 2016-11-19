#version 150 core

in vec3 a_Pos;
in vec2 a_TexCoord;
in vec3 a_Translate;
in int a_Height;
flat out int v_Height;
out vec2 v_TexCoord;

uniform Locals {
	mat4 u_Transform;
};

void main() {
    v_TexCoord = a_TexCoord;
    v_Height = a_Height;
    gl_Position = u_Transform * vec4(a_Pos * (1<<a_Height) + a_Translate, 1.0);
    gl_ClipDistance[0] = 1.0;
}
