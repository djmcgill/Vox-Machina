#version 150 core

in vec3 a_Pos;
in vec2 a_TexCoord;
in vec3 a_Translate;
in float a_SideWidth;
out float v_SideWidth;
out vec2 v_TexCoord;

uniform Locals {
	mat4 u_Transform;
};

void main() {
    v_TexCoord = a_TexCoord;
    v_SideWidth = a_SideWidth;
    gl_Position = u_Transform * vec4(a_Pos * a_SideWidth + a_Translate, 1.0);
    gl_ClipDistance[0] = 1.0;
}
